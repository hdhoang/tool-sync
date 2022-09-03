# tool-sync

[![GitHub CI](https://github.com/chshersh/tool-sync/workflows/CI/badge.svg)](https://github.com/chshersh/tool-sync/actions)
[![Latest GitHub release](https://img.shields.io/github/v/release/chshersh/tool-sync)](https://github.com/chshersh/tool-sync/releases/latest)
[![MPL-2.0 license](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](LICENSE)

`tool-sync` is a CLI tool for installing your other favourite tools from GitHub Releases.

![tool-sync demo](./images/demo.gif)

> ℹ️ **DISCLAIMER:** `tool-sync` is developed and maintained in free time
> by volunteers. The development may continue for decades or may stop
> tomorrow. You can use
> [GitHub Sponsorship](https://github.com/sponsors/chshersh) to support
> the development of this project.

## What it really does?

`tool-sync` embraces the idea that configuring your personal development
environment should be as easy as possible. And the life is pretty easy when all
the tools are simple executables.

**So why not simply download all executables you use and put them in one place???** 😱

With `tool-sync`, you can install all the tools you use by following three
simple steps:

1. [Install `tool-sync`](#install).
2. [Configure](#configure) `tool-sync` by listing all the tools you need and
   specifying where to put them.
3. Run `tool sync`.

That's all! 🥳

Then `tool-sync` does the following:

* Fetches the information about tools from GitHub Releases
* Automatically guesses the asset name from your OS for common tools
* Downloads and unpacks assets
* Copies binaries from unpacked assets to the location of your choice

## Features

`tool-sync` has several distinguished features that allows you to manage your
personal toolbox easily:

* Installs the latest version of tools by default. You can easily update all
  your tools with a single command!
* Supports common tools that you can easily install without extra configuration
* Automatically guesses asset name from your current OS
* Configures via a simple TOML file

## Install

### From releases (recommended)

You can install `tool-sync` directly from GitHub releases in a few steps:

1. Go to the [latest release](https://github.com/chshersh/tool-sync/releases/latest).
2. Download an asset for your OS.
3. Unpack the `tool` executable to a desired location.

<!-- Good news, you only need to do this once! `tool-sync` will manage future
installations of itself (if you add it to your config). -->

### From crates

You can use `cargo` to install the latest published version of `tool-sync` from crates:

```shell
cargo install tool-sync
```

### From sources

You can install the latest version of `tool-sync` from sources (requires `git`
and `cargo`):

```shell
git clone https://github.com/chshersh/tool-sync
cd tool-sync
cargo build --release
./target/release/tool --version
```

## Configure

`tool-sync` reads configuration from a file in TOML format. An example
configuration file is shown below:

```toml
# a directory to store all tools
store_directory = "~/.local/bin"

# the following tools will be installed in 'store_directory'
[bat]
[difftastic]
[exa]
[fd]
[ripgrep]
```

By default `tool-sync` reads configuration from `~/.tool.toml` but you can put
the content in any place and specify the path via the `--config` flag.

You can also quickly copy the above configuration to the default path by running
the following command (Unix-only):

```shell
curl https://raw.githubusercontent.com/chshersh/tool-sync/main/example-tool-sync-config.toml > ~/.tool.toml
```

The above example config lists some tools natively supported by `tool-sync` and
therefore they don't require extra configuration.

To specify a tool not supported by `tool-sync`, add a TOML table entry and list
all the required fields like in the example below:

```toml
[tokei]
owner    = "XAMPPRocky"  # GitHub username
repo     = "tokei"       # GitHub repository
exe_name = "tokei"       # Executable name inside the asset

# uncomment to download a specific version or tag
# tag = "12.1.1"

# Asset name to download on linux OSes
asset_name.linux = "x86_64-unknown-linux-musl"

# uncomment if you want to install on macOS as well
# asset_name.macos = "apple-darwin"

# uncomment if you want to install on Windows as well
# asset_name.windows = "x86_64-pc-windows-msvc"
```

> ℹ️ `tool-sync` searches asset name using the _substring search_. That's why
> you don't need to specify the full asset name in the config, only the minimal
> part required for identifying the asset. However, `tool-sync` doesn't guarantee
> you to find the asset you need if multiple assets from the GitHub release match
> the substring.

All fields in each tool section are

+ **required for unknown tools,**
+ _optional for known tools._

This means that you can override only some of the fields for known tools.

This can be helpful if e.g. you want to install a custom version of `ripgrep`
from a forked repository. To do this, specify only the repository owner in the
config:

```toml
[ripgrep]
owner = "me"
```

## Usage

Install all the tools specified in `~/.tool.toml`:

```shell
tool sync
```

Install all the tools from config in a different location:

```shell
tool --config=path/to/my/config.toml sync
```

Run `tool --help` for more details.

> :octocat: If you hit the limit for downloading assets or want to download
> assets from private repositories, 
> [create a personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
> and export it as the `GITHUB_TOKEN` environment variable.

## Alternatives

This section contains `tool-sync` comparison to existing alternatives:

1. **Manual download**. You can download GitHub releases manually without using
   any extra tools.
   
   + **Pros**
     + No extra tools required, only your browser and unpack utility
   + **Cons**
     + Tedious manual process

2. **GitHub CLI**. You can download assets from releases using 
   [the GitHub CLI tool `gh`][gh].

   ```shell
   gh release download --repo chshersh/tool-sync v0.0.0 --pattern='*linux*'
   tar -xvf tool-x86_64-unknown-linux-gnu.tar.gz
   ./tool --version
   ```

   + **Pros**
     + Using a more common tool (that you probably have)
   + **Cons**
     + Can't download multiple tools with a single command
     + Can't guess the asset name by your OS

3. [**dra**][dra]. `dra` is the closest alternative to `tool-sync`. It's a CLI
   tool, written in Rust, that allows downloading individual releases easily.

   + **Pros**
     + Convenient interface for downloading a single release
   + **Cons**
     + Can't download multiple tools with a single command
     + Can't guess the asset name by your OS

4. [**home-manager**][home-manager]. Home Manager provides a full-features
   solution for managing a user environment using the Nix package manager.

   + **Pros**
     + Supports more than downloading tools from GitHub Releases
     + Access to the bigger Nix ecosystem
   + **Cons**
     + More complicated solution
     + Requires learning and using Nix

[gh]: https://github.com/cli/cli
[dra]: https://github.com/devmatteini/dra
[home-manager]: https://github.com/nix-community/home-manager

## For contributors

Check [CONTRIBUTING.md](https://github.com/chshersh/tool-sync/blob/main/CONTRIBUTING.md)
for contributing guidelines.

## Development

### Build

Use `cargo` to build the project and run all tests:

```shell
cargo build
cargo test
```

### Adding a new tool

`tool-sync` contains [a database of common tools][db] and provides easier
support for them. It's possible to add more tools (and you can suggest them!).
The following list contains guidelines for including a new tool. They don't
serve as gatekeeping criteria but more as points system:

* 6 months passed since the tool release
    + So that the database won't be populated with fresh tools that are never
      supported
* At least 3 releases
    + To ensure stable naming scheme for assets
* Commonly used tool
    + `tool-sync` strives to be generic so it might not want to support a DNA
      analysis CLI tool which is useful only for a specific group
* The `tool-sync` author find the tool helpful
    + In the end, there're people behind `tool-sync` who maintain this project
      while the rest of the world benefits from it for free. At least, `tool-sync`
      authors decide what they want to use and whether they want to support a tool indefinitely.

[db]: https://github.com/chshersh/tool-sync/blob/main/src/sync/db.rs
