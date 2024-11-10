# Very rough open-webui cli

This is a very rough/dumb cli for open-webui. Why? Cause I need to use things outside of a webbrowser session and this makes it easier to script with than using curl all over and jq.

Have to setup an api token to use as this is doing bearer access token as auth. You'll need to either set your token and uri via environment variables or switches.

```sh
export TOKEN=sk-40cfb1e4ac104872bbb318e9b3df3057
export URI=open-webui.home.arpa
```

Note: I might end up changing the cli ux as I implement more of the apis. I'm trying to make the cli more... cli-y. Ideas on ux welcome. Function over form for now.

## Implemented functions

Note the cli is mostly self documenting, run it without arguments for details.

But in general you can at least do:
- Knowledge collection deletion/creation/add files
- Rag file upload/delete
- Chat with an llm whilst using a collection or file as input

Example (note for now you have to have the models loaded already, this is next on the list to implement):

Chat with a model and get result back (answer is sus but its an example):

```sh
$ open-webui-cli list models
codellama:latest
llama3:latest
mistral:latest
nomic-embed-text:latest
gemma:latest
nemotron-mini:latest
llama3.2:latest
$ open-webui-cli chat --model llama3.2:latest --prompt "how many roman imperators built walls (answer with the number only)"
21
```

See how many kind of color dragons there are in DND (allegedly I don't know I'm not a DND fan so I assume the AI is perfect entirely correct. All I know is there is a lot of them of all colors like a rainbow):

```sh
$ curl -sLO https://media.wizards.com/2018/dnd/downloads/DnD_BasicRules_2018.pdf
$ open-webui-cli new file --file DnD_BasicRules_2018.pdf
84bc4e62-c952-4904-84d1-6341a1ae1036
$ open-webui-cli chat --model llama3.2:latest --prompt "how many color dragons are in dnd (answer with the color names only)" --file 84bc4e62-c952-4904-84d1-6341a1ae1036
There are 3 color dragons: Blue, Green, and Red.
$ open-webui-cli chat --model llama3.2:latest --prompt "how many color dragons are in dnd (answer with the color names only)"
Red, Blue, Green, White, Black, Copper, Gold, Silver
```

Which is true? Only the shadow knows or a DND nerd. Wonder why there aren't titanium dragons or other metals. Probably not a rat hole I want to go down.

## Installation

- Well cargo build/install it for now
- If you have nix and direnv you can just `direnv allow` and then `nix build` to build binaries.
- Or if you use nix you can just run it this way `nix run github:mitchty/open-webui-cli -- args`

## TODO

A metric ton of stuff ordered by priority:
- llm related actions
- anything else I end up using
- musl static build with github release/action for linux x86/arm
- chat api so you can use ^^^
- delete/add llm models
- define new models based on existing
- ?
- More auth types?
- Better error handling? (I cheaped out on a lot of this for now, using anyhow so should be able to improve it with some effort)
- macos/windows binaries for releases too?
- unit tests? Not sure how I want to do this for the moment
