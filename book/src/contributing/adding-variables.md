# Adding A New Variable

You shouldn't need any code to add a new variable, just update [responses.json](https://github.com/Gankra/cargo-mommy/blob/main/responses.json) and the docs~


## Editing responses.json

* Add [the new variable to responses.json's "vars" object](./concepts.md#responsesjson)~
* Fill in [the default values](./concepts.md#variables)~
* If NSFW, be sure to set "spiciness"~
* Use the variable in [existing or new messages](./concepts.md#variables)~

For instance, here's a minimal example "furniture" variable that is set with `CARGO_MOMMYS_FURNITURES`~ 🪑

```json
{
    "vars": {
        "furniture": {
            "defaults": ["chair", "desk"]
        },
    }
}
```

and here's how you might use it~

```json
{
    "moods": {
        "chill": {
            "positive": [
                "thanks for helping build {role}'s {furniture}~"
            ],
            "negative": [
                "ouch! {role} stubbed {pronoun} toe on the {furniture}!"
            ]
        }
    }
}
```

That's it, no code needs to be changed~ 💕


## Updating The Docs

Add the variable to the [SFW variable docs](../customize/roles-and-pronouns.md.md) or the [NSFW variable docs](../customize/nsfw.md#variables).