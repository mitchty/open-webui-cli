# Very rough open-webui cli

This is a very rough/dumb cli for open-webui. Why? Cause I need to use things outside of a webbrowser session and this makes it easier to script with than using curl all over and jq.

Have to setup an api token to use as this is doing bearer access token as auth. You'll need to either set your token and uri via environment variables or switches.

```sh
export TOKEN=sk-40cfb1e4ac104872bbb318e9b3df3057
export URI=open-webui.home.arpa
```

Note: I might end up changing the cli ux as I implement more of the apis. I'm trying to make the cli more... cli-y. Ideas on ux welcome. Function over form for now.

## Implemented functions

Note the cli is mostly self documenting, run it without arguments for details. It also follows "no news is good news" as a general principle. If the return code is 0 everything should be ok.

But in general you can at least do:
- Knowledge collection deletion/creation/add files
- Rag file upload/delete
- Chat with an llm whilst using a collection or file as input

Example (note for now you have to have the models loaded already, this is next on the list to implement):

Chat with a model and get result back (answer is sus but its an example):

```sh
$ open-webui-cli list models
$ open-webui-cli pull model --name llama3.2:latest
$ open-webui-cli list models
llama3.2:latest
$ open-webui-cli chat --model llama3.2:latest --prompt "how many roman imperators built walls (answer with the number only)"
21
```

See how many kind of color dragons there are in DND (allegedly I don't know I'm not a DND fan so I assume the AI is perfect at everything. All I know is there is a lot of them of all colors like a rainbow based off conversations I've overheard DND friends have):

```sh
$ curl -sLO https://media.wizards.com/2018/dnd/downloads/DnD_BasicRules_2018.pdf
$ open-webui-cli upload file --name DnD_BasicRules_2018.pdf
84bc4e62-c952-4904-84d1-6341a1ae1036
$ open-webui-cli chat --model llama3.2:latest --prompt "how many color dragons are in dnd (answer with the color names only)" --file 84bc4e62-c952-4904-84d1-6341a1ae1036
There are 3 color dragons: Blue, Green, and Red.
$ open-webui-cli chat --model llama3.2:latest --prompt "how many color dragons are in dnd (answer with the color names only)"
Red, Blue, Green, White, Black, Copper, Gold, Silver
```

Which is true? Only the shadow knows or a DND nerd. Wonder why there aren't titanium dragons or other metals. Probably not a rat hole I want to go down.

We can similarly use collections instead of individual files:

```sh
$ curl -sLO https://media.wizards.com/2018/dnd/downloads/DnD_BasicRules_2018.pdf
$ curl -sLO http://media.wizards.com/2014/downloads/dnd/PlayerDnDBasicRules_v0.2_PrintFriendly.pdf
$ open-webui-cli upload file --name DnD_BasicRules_2018.pdf
c90d289c-0009-4150-8ebb-6f5a0628607d
$ open-webui-cli upload file --name PlayerDnDBasicRules_v0.2_PrintFriendly.pdf
f9923666-d337-4514-8868-6a725967220b
$ open-webui-cli new collection --name dnd --description "dnd bag of holding"
1ae97ee3-c7c0-400d-9b5c-b9939def311f
$ open-webui-cli link collection --id 1ae97ee3-c7c0-400d-9b5c-b9939def311f --file-id f9923666-d337-4514-8868-6a725967220b
1ae97ee3-c7c0-400d-9b5c-b9939def311f
$ open-webui-cli link collection --id 1ae97ee3-c7c0-400d-9b5c-b9939def311f --file-id c90d289c-0009-4150-8ebb-6f5a0628607d
1ae97ee3-c7c0-400d-9b5c-b9939def311f
$ open-webui-cli chat --model llama3.2:latest --prompt "how many color dragons are in dnd (answer with the color names only)" --collection 1ae97ee3-c7c0-400d-9b5c-b9939def311f
There is only one color dragon mentioned, a red dragon.
$ open-webui-cli chat --model llama3.2:latest --prompt "summarize the pdfs in this collection" --collection 1ae97ee3-c7c0-400d-9b5c-b9939def311f
The PDF collection contains rules and information for Dungeons & Dragons (D&D) players. The content is divided into two main sections: "DnD Basic Rules_2018.pdf" and "PlayerDnDBasicRules_v0.2_PrintFriendly.pdf".

The first section, "DnD Basic Rules_2018.pdf", appears to be a basic ruleset for the game, covering various aspects such as character creation, combat, and gameplay mechanics.

The second section, "PlayerDnDBasicRules_v0.2_PrintFriendly.pdf", provides more detailed information on tools, equipment, and skills for players. The content includes lists of items such as Smith's tools, Tinker's tools, Weaver's tools, and others, along with their prices and weights.

Some notable items mentioned in the document include:

* Forgery kit: a box containing papers, pens, ink, seals, and other supplies to create convincing forgeries.
* Gaming set: an item that includes game pieces, such as dice and decks of cards, which allows players to add their proficiency bonus to ability checks when playing with that set.

Overall, the PDF collection provides a comprehensive resource for D&D players, covering various aspects of the game.
```

## Installation

Snag it from the releases page. or if you're building from source then cargo build/install as is tradition.

If you have nix and direnv you can just `direnv allow` and then `nix build` to build binaries. Or if using nix you can just build/run it directly like any other flake. `nix run github:mitchty/open-webui-cli -- args`

## TODO

A metric ton of stuff ordered by priority (roughly):
- anything else I end up abusing
- define new models based on existing
- profit?
- More auth types than the api token?
- Better error handling? (I cheaped out on a lot of this for now, using anyhow so should be able to improve it with some effort)
- unit tests? Not sure how I want to do this for the moment
- Open up issues for anything else. Some of these apis are rather ill defined, or the openapi generator goes insane or both at times.
