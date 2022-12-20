use rustc_version::Version;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let version = rustc_version::version().unwrap();

    // How annoying it is to check rust versions! Why do we do this?
    // The stable ExitCode API is locked behind `feature(process_exitcode_placeholder)`
    // and Termination behind `feature(termination_trait_lib)` prior to 1.61.0.
    // These provide nice functionality, but keep legacy developers away from mommy.
    // We provide a "compat" version of the program that's basically the same,
    // but avoids using any ExitCode functionality.
    if version < Version::new(1, 61, 0) {
        println!("cargo:rustc-cfg=compat");
    }
}
