# Very rough open-webui cli

This is a very rough/dumb cli for open-webui. Why? Cause I need to use things outside of a webbrowser session and this makes it easier to script with than using curl all over and jq.

Have to setup an api token to use underneath this is doing bearer access token as auth.

## Implemented apis

### ollama

- /ollama/v1/models GET -> llm list
- /ollama/api/generate POST -> llm query
- /webui/api/v1/files POST -> rag upload

Too many more to implement yet. Pr's welcome. I'll likely only implement what I need.

Abuse at your own risk.

## Installation

- Well cargo build/install it for now
- If you have nix and direnv you can just `direnv allow` and then `nix build` to build binaries.
- Or if you use nix you can just run it this way `nix run github:mitchty/open-webui-cli -- args`

## TODO

A metric ton of stuff ordered by priority:
- musl static build with github release/action for linux x86/arm
- rag api
- chat api so you can use ^^^
- delete/add llm models
- define new models based on existing
- ?
- More auth types?
- Better error handling? (I cheaped out on a lot of this for now)
- macos/windows binaries for releases too?
- unit tests? Not sure how I want to do this for the moment
