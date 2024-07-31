use fastrand::Rng;
use std::time::{Duration, SystemTime};
use std::path::PathBuf;

/// The lock file name
const LOCK_FILE_NAME: &str = "MOMMY.lock";

/// The beg file name
const BEG_FILE_NAME: &str = "MOMMY-PLEASE.time";

/// whether mommy needs her pet to beg, and how to create a lock if they do.
pub struct BegCtx {
    /// Whether or not mommy requires begging
    pub needs: NeedsBeg,

    /// Path to the lock file that may or may not exist
    pub path: PathBuf,
}

impl BegCtx {
    #[must_use]
    pub fn is_needed(&self) -> bool {
        !matches!(self.needs, NeedsBeg::NotNeeded)
    }

    /// Remove a lock file
    pub fn remove_lock(&mut self) -> Result<(), std::io::Error> {
        match std::fs::remove_file(&self.path) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            err => err,
        }
    }
}


/// Mommy should be able to tell if this is her first time asking for a pet to beg~
pub enum BegKind {
    RequestFirstBeg,
    EnforceFirstBeg,
    RequestBegMore,
}

pub enum NeedsBeg {
    NotNeeded,
    Needed(BegKind),
}

/// does mommy need a little extra~?
pub fn check_need_beg(rng: &Rng, mut begging: u8, beg_half_life: u16, stubborn_chance: u8) -> anyhow::Result<BegCtx> {
    let lock_file_path = {
        let mut file = home::cargo_home()?;
        file.push(LOCK_FILE_NAME);
        file
    };

    // Fast path if mommy's pet is always good~
    if beg_half_life == 0 {
        return Ok(BegCtx {
            needs: NeedsBeg::NotNeeded,
            path: lock_file_path,
        });
    }

    // Check if they begged using a file
    let beg_file_path = {
        let mut file = home::cargo_home()?;
        file.push(BEG_FILE_NAME);
        file
    };
    let recent_beg = std::fs::OpenOptions::new()
        .write(true)
        .create(begging > 0)
        .open(&beg_file_path);
    let elapsed = match recent_beg {
        Ok(recent_beg) => {
            if begging > 0 {
                recent_beg.set_modified(SystemTime::now())?;
                Duration::new(0, 0)
            } else {
                recent_beg.metadata()?.modified()?.elapsed()?
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // Pet has never begged before, or the previous beg was revoked.
            // Mommy will be generous and consider her pet's previous beg to be 1 year ago.
            Duration::new(365 * 24 * 60 * 60, 0)
        }
        Err(err) => return Err(anyhow::Error::from(err))
    };

    // Was pet's previous begging recent enough?
    let decay_rate = std::f64::consts::LN_2 / beg_half_life as f64;
    let probability = f64::exp(-decay_rate * elapsed.as_secs_f64());
    let beg_is_recent_enough = rng.f64() < probability;

    // called without begging, but begging file could be externally refreshed
    if beg_is_recent_enough && begging == 0 {
        begging += 1;
    }

    // Unconditionally create lock file to try and mitigate funny toctou
    let maybe_lock = std::fs::OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(&lock_file_path);

    let needs = match (beg_is_recent_enough, maybe_lock) {

        // previous beg is recent enough
        // mommy did not request begging anyway
        (true, Ok(_)) => NeedsBeg::NotNeeded,

        // previous beg is not recent enough
        // mommy will make her first request
        (false, Ok(_)) => {
            // Delete latest beg
            let _ = std::fs::remove_file(&beg_file_path);
            // Request first beg
            NeedsBeg::Needed(BegKind::RequestFirstBeg)
        }

        // previous beg is not recent enough
        // even AFTER mommy specifically reminded her pet
        (false, Err(err)) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            // Delete latest beg
            let _ = std::fs::remove_file(&beg_file_path);
            // Enforce first beg
            NeedsBeg::Needed(BegKind::EnforceFirstBeg)
        }

        // previous beg is recent enough
        // But because mommy had to remind her pet so much,
        // maybe she's feeling stubborn and wants more...
        (true, Err(err)) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            let mut beg_more = true;
            while begging > 0 && beg_more {
                let stubborn_pick = rng.u8(..100);
                beg_more = stubborn_pick < stubborn_chance;
                begging -= 1;
            }
            if beg_more {
                // Delete latest beg
                let _ = std::fs::remove_file(&beg_file_path);
                // Require more begging
                NeedsBeg::Needed(BegKind::RequestBegMore)
            } else {
                // Okay, mommy is satisfied
                NeedsBeg::NotNeeded
            }
        }
        (_, Err(err)) => return Err(anyhow::Error::from(err))
    };
    return Ok(BegCtx { needs, path: lock_file_path });
}