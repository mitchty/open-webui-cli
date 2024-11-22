# Changelog

All notable changes to this project will be documented in this file. Note to simplify the changelog the cli is referred to in examples as simply `cli`.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.1.2] - 2024-11-22

Minor update, removes --port and other switches in lieu of simply URI/--uri. The default is http://localhost if not specified.

To update simply change what uri was, which was more fqdn to:

```sh
export URI=http(s)://example.tld:1234
```

Other env vars remain the same.

Added `--insecure` to allow for ignoring remote ssl certificates.

## [0.1.1] - 2024-11-18

Removed unneeded switches like `--name` and `--id` in many spots so where you would do:

```sh
cli upload file --name justonefile
```

You can now instead do:

```sh
cli upload file onefile twofile threefile fourandmore
```

This is so you can chain commands together more easily and also make stuff like:

```sh
cli upload file $(find /dir -name "*.sh")
cli delete models $(cli list models)
```

"just work(tm)"


Added `--system` to the chat prompt and made `--prompt` able to be specified multiple times like the underlying api allows for. Why you would want to do that is beyond me but you can. You do you.

## [0.1.0] - 2024-11-11

Initial release. Full of bugs and shenanigans.
