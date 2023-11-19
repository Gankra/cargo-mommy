# Roles And Pronouns

Mommy knows you might not be a "girl", and might not want her to be a "mommy" or a "her"~

Mommy will read the following environment variables to make her messages better for you~ ❤️

* `CARGO_MOMMYS_LITTLE` - what to call you~ (default: "girl")
* `CARGO_MOMMYS_PRONOUNS` - what pronouns mommy will use for themself~ (default: "her")
* `CARGO_MOMMYS_ROLES` - what role mommy will have~ (default "mommy")
* `CARGO_MOMMYS_EMOTES` - what emotes mommy will have~ (default "❤️/💖/💗/💓/💞")

See [the NSFW docs for additional spicier variables](./nsfw.md#variables)~

All of these options take ISO Standard Pronoun Syntax, which is to say a slash-delimited list like "she/he/they", "girl/boy", or "daddy". Mommy will randomly select one of them whenever she talks to you~

For example, the phrase "mommy loves her little girl~ 💞" is "CARGO_MOMMYS_ROLE loves CARGO_MOMMYS_PRONOUNS little CARGO_MOMMYS_LITTLE~"

So if you set `CARGO_MOMMYS_ROLES="daddy"`, `CARGO_MOMMYS_PRONOUNS="his/their"`, and `CARGO_MOMMYS_LITTLE="boy/pet/baby"` then you might get any of

* daddy loves their little boy~ ❤️
* daddy loves his little pet~
* daddy loves their little baby~ 💗

And so on~ 💓
