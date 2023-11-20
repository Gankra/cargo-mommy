# Adding A New Mood

You shouldn't need any code to add a mood, just update [responses.json](https://github.com/Gankra/cargo-mommy/blob/main/responses.json) and the docs~

## Editing responses.json

* Add [the new mood to responses.json's "moods" object](./concepts.md#responsesjson)~
* Fill in as many messages as you can think of for [the message pools](./concepts.md#message-pools)~
* If NSFW, be sure to set "spiciness"~

For instance, here's a minimal example (SFW) "sleepy" mood~ 💤

```json
{
    "moods": {
        "sleepy": {
            "positive": [
                "that almost makes {role} want to get out of bed...",
                "*yawns*\ngood work~"
            ],
            "negative": [
                "{role} thinks {pronoun} little {affectionate_term} might also be too tired~",
                "let's just take a nap, ok~?"
            ],
            "overflow": [
                "{role} did too much and is going to bed..."
            ]
        },
    }
}
```

That's it, no code needs to be changed!


## Updating The Docs

Add the mood and some example outputs to the [SFW mood docs](../customize/moods.md) or the [NSFW mood docs](../customize/nsfw.md#moods).