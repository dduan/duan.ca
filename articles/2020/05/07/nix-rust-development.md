# Naive NixOS Rust Development
2020-05-07T11:04:58-07:00
tags: Nix, NixOS, Rust, RLS

tl;dr: To work on Rust project with nix-shell, rls and extensions such as
`rust-analysis`, `rust-src`, without caring too much about specific Rust
toolchain version (except for it being `stable`), use the following `shell.nix`:

```nix
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];}
  );
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [ rustup ruststable ];
  }
```

When you have a Nix hammer, everything looks like a Nix expression.

Having used NixOS on a real PC for a number of days, this is the impression
I get from the world of Nix. Unfortunately, so far, it's been a negative for me.

One of the most exciting thing I want to use Nix for is to bootstrap development
environment with `nix-shell`. I imagined it to be similar to using [pipenv][]
with Python, except for everything. Well, I've since learned that it's not true
(yet?) for many reasons.


Modern programming languages come with their own attempt at *reproducibility*.
Some does it better than others. To make it concrete, I'm talking about things
like [Stack][] for Haskell or [rustup][] for Rust: given the source code, how do
I make it build in the way the project intended? What's the correct version of
the compiler, runtime, and tools that works best with this revision of the
source code? The common solution usually follows this pattern: as author of
a project, specify as much as you can, the environment best suited for the
current state of the project. As a "builder", use a *single program* that's
capable of updating itself, as well as ensuring that the project builds exactly
as specified, including managing the compiler/runtime/tooling versions, etc.

This *single program*'s role is very much the same as the Nix system, except the
latter is independent of programming languages: `rustup` installs Rust, so does
Nix. That's a bad thing. As a package manager, Nix either have to tightly
integrate with each of these other package managers, leveraging their evolving
behaviors to give its user the build environment; or, it must replace them.
The former is impractical; the latter, well, sucks.


Back to reality. This is the experience I want to have with NixOS: Some programs
I use daily such as Alacritty, NeoVim, Firfox, etc, are installed globally and
readily available. They are part of my `/etc/nixos/configuration.nix`. So far so
good. Now, I regularly program in a few languages. For each of the project, I'd
like to have a `shell.nix` that brings in its compilers, libraries, LSP servers,
etc. This is what `nix-shell` is supposed to give me! This is known as the "per
project" setup.

Let's see: with Rust, that means `rustc` (compiler), `cargo` (package manager),
`rls`, `rust-src` and `rust-analysis` (LSP). In macOS, I'd install all of these
globally with `rustup`. In NixOS...well, I can ask for `rustup` for my project:

```nix
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust";
  nativeBuildInputs = [ rustup ];
}
```

...which gives me `rustup` and nothing else. That's right, you don't even get
a `rustc` after running `nix-shell`. But `rustup` can get you everything else,
all I need to do is ask. Hmm, do I need to run a series of set-up commands with
`rustup` every time I enter the environment? No? I just need to run it the first
time? Until the cached tools get deleted by some garbage collection mechanism?
That seems unsatisfying, doesn't it?

Instead of `rustup`, I could also ask Nix to use the `rustc`/`cargo`/`rls` it
knows about directly. This is marginally better. Except I still need `rust-src`
and `rust-analysis` for my needs. As far as I can tell, these [RLS][] components
are out of Nix's control (as of today).

Everywhere on the internet I looked, for every problem that
Nix-the-package-manager doesn't work out-of-the-box, there's someone responding
along the line of "you can write some Nix expression yourself". In other words,
Nix-the-language is powerful enough to solve it, probably. In the case of Rust,
luckily, Mozilla wrote enough Nix expressions for us and provides them via an
[overlay][Mozilla overlay]. These expressions are rich enough to meet my needs.
As you can see in the tl;dr at the top, when entering the development
environment, nix-shell would: download the overlay's source code (from the
internet, or local cache), load the expression it includes, mix in my
customization, and execute it.

That marks the end of my search. I like the final solution because it's mostly
"vanilla" Nix and doesn't require me to mess with a bunch of other tools. For
solutions that do, read [this][How I Start: Nix].


***

At end of the day, my needs are pretty basic: consistency from rustup and
convenience from nix-shell. I didn't need to pin the compiler to a specific Rust
release, or checksum the final build output.

I'm very new to both technologies so there may be a follow-up post sometime in
the future.

[pipenv]: https://pipenv-fork.readthedocs.io/en/latest/
[Stack]: https://haskellstack.org
[rustup]: https://rustup.rs/
[RLS]: https://github.com/rust-lang/rls
[Mozilla overlay]: https://github.com/mozilla/nixpkgs-mozilla
[How I Start: Nix]: https://xeiaso.net/blog/how-i-start-nix-2020-03-08/
