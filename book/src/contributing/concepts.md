# Concepts

At her heart, mommy exists to invoke cargo, check if the result was a success or failure, and print an appropriate message~
Printing that message involves reading a bunch of configuration from env-vars to define pools of values to randomly select from, then:

1. randomly [decide what mood the response should have](#moods)
2. use the success/failure state to select which pool of answers from that mood to use (positive, negative, overflow)
3. randomly select a message from that pool

cargo-mommy has 3 major components~

1. [responses.json, where data lives](https://github.com/Gankra/cargo-mommy/blob/main/responses.json)
2. [build.rs, that digests that data and converts it to rust code](https://github.com/Gankra/cargo-mommy/blob/main/build.rs)
3. [main.rs, that handles execution](https://github.com/Gankra/cargo-mommy/blob/main/src/main.rs)

The build.rs is extremely extra, but it's actually kind of useful for making it tolerable to [disable nsfw features](../customize/never-nsfw.md). In theory it also makes cargo-mommy very efficient and fast, which, again, is very extra.



## Responses.json

The ""schema"" of responses.json is currently the following (`$SOME_VALUE` here is an ~arbitrary string):

```text
{
    "moods": {
        "$MOOD": {
            "positive": ["$GOOD_MESSAGE", ...]
            "negative": ["$BAD_MESSAGE" ...]
            "overflow": ["$E_TOO_MANY_MOMMY_MESSAGE", ...]
            "spiciness": "chill" | "thirsty" | "yikes" | <defaults to "chill">,
        }
    }
    "vars": {
        "$VAR": {
            "defaults": ["$VALUE", ...]
            "env_key": "$KEY" | <defaults to $VAR>
            "spiciness": "chill" | "thirsty" | "yikes" | <defaults to "chill">,
        }
    }
}
```




### Moods

Moods contain [pools of messages](#message-pools) with a particular feeling/intensity. This was originally introduced to allow the user to opt into [nsfw functionality](../customize/nsfw.md). Its functionality is more general than that, and we'd be happy to have more sfw moods like "furry" or whatever.

The "chill" mood is the default mood, and is assumed to exist.

Spiciness is used to determine whether a mood's contents should be considered "nsfw" content, either "chill", "thirsty", or "yikes". Everything spicier than "chill" is considered "nsfw". There's no hard and fast rules here, just vibes. Spicier moods also gain access to spicier [variables](#variables).

There are 2 moods that have the same names as their equivalent spiciness level, which may help determine how nsfw a new mood is.




### Message Pools

Each mood must specify 3 pools of [messages](#messages) to select from:

* "positive" messages appear when the cargo command mommy invoked was a success
* "negative" messages appear when the cargo command mommy invoked was a failure
* "overflow" messages appear when it was recursively invoked too many times (to break out of infinite loops from misconfiguration)



### Messages

Messages are strings that optionally contain [`{variables}`](#variables) that need to be substituted (emotes don't appear in the message, they're just auto-applied to the end of every message).

Some examples:

"{role} thinks {pronoun} little {affectionate_term} earned a big hug~"

Becomes something like:

"Mommy thinks her little girl earned a big hug~ ❤️"

Messages have the following conventions:

* `{role} ends {pronoun} messages with tildes~`
* `*{role} performs actions with asterisks*`
* `*{role} combines both*\nwith newlines~`





### Variables

Variables are a pool of values that will be randomly selected from, and substituted into the templates.

The variables "mood", "emote", "pronoun", and "role" are explicitly named in cargo_mommy's main.rs, and are assumed to appear at the start of `vars` in that exact order.

Spiciness is used to determine whether a variable's contents should be considered "nsfw" -- either "chill", "thirsty", or "yikes". Everything spicier than "chill" is considered "nsfw". There's no hard and fast rules here, just vibes. Spicier variables should only ever be used by spicier [moods](#moods).

env_key is used to define a SCREAMING_CASE env-var `CARGO_{TRUE_ROLE}S_{ENV_KEY}S` (note the two extra S's!). For instance the "mood" key would be `CARGO_MOMMYS_MOODS`.

If a variable's env-var isn't set, it will use its defaults as the pool. ("role" has no default value in responses.json because it has special logic to default to the [True Role](../customize/true-roles.md) when no custom roles are set.)

If a variable's env-var *is* set, mommy will parse it as a ISO Standard Pronouns List, which is to say a slash-delimited list like "she/he/they", "girl/boy", or "daddy", and use those values as the pool.

Each time mommy encounters a variable that needs to be substituted in a message, it will randomly select a new value from the pool.

At the end of a message, mommy will randomly decide whether to include an emote.

