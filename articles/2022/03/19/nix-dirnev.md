# Flake For Non-Nix Projects
2022-03-19T00:37:01-07:00
tag: Nix, home-manager, flake

The ideal outcome of using Nix or home-manager is to have as many things as possible under their control.
Among these things are developer tools for coding. You may need `cargo`, and `rust-analyzer` for Rust projects;
`ipython`, and `python-lsp-server` for Python projects, etc.

Of course, the simplest solution is to have them all installed and configured in home-manager or the OS level.
But, over time, you'll end up with a ton of stuff.

Maybe that won't bother you. Fine, but this approach is at odds with some convention of the Nix community. You
see, there's this command called `nix-shell`, or `nix develop`, which sets up a environment with certain
packages, with the expectation that you only need them sometimes, usually within the context of developing
a certain software project. 

So, arguably, a better alternative to installing everything you possibly need is to keep only the essential
tools you always need, regardless of what you are working on. Things like `neovim`, `git`, `ripgrep`, etc.
When a project demands things such as `rust-analyzer`, running `nix develop` should set it up for you.

How do we implement that? In a imaginary world where every single software project is built with a flake.nix,
the `devShell` property should provide everything the software project owner expect you to need. And you just
`cd` into the root of the project, run `nix develop`, and you are off to the races.

Several problems:

1. running `nix develop` is repetitive, and therefore, annoying.
2. the owner of a flake.nix may have drastically different taste for the best dev tools for their project
   (bet!).
3. we don't live in a world built with Nix flakes.

Yikes.

To solve `1`, enter [direnv][]. It's a piece of software that let you create a environment for a directory,
and automatically switch to it when you enter said directory. The "environment" could contain envvars, and,
you guessed it: a Nix developer shell. If you have a flake.nix, add `use flake` in your project directory's
`.envrc` file, direnv will automatically call `nix develop` the next time you `cd` in. Neat!

To make this work in practice, you'll want to add `.envrc` and `.direnv/` to your global git ignore list, as
they are personal preferences that probably shouldn't end up in git history.

Ok, we are so close to solve problem `2`, and `3` now. In short, `nix develop` may set up **a** environment,
but it may not be **the** environment that suits you the best. [nix-direnv][] extends direnv to save us:

```
use flake path/to/flake#target
```

... with this in `.envrc`, direnv will set up the Nix environment according to `path/to/flake#target`. That
could point to any location on your hard drive! You can have a flake.nix whose `devShell` has `cargo`, and
`rust-analyzer`. You can have another with `ipython`, and `python-lsp-server`. Mix and match to your liking to
the infinite degree...

For now, I've decided to give nix-direnv a try. Alongside my home-manager configuration, I've also included
extra flakes for generic Python/Rust projects, and specific projects that may require a mix of tools. A lot of
the project I work on don't use flake as their package manager. With this approach, I get to customize my
setup for them each, and stay in the comfort of Nix and home-manager.

direnv and nix-direnv can be configured together with home-manager. To achieve everything mentioned in this
article, including direnv's shell integration (Fish, for me), it's as simple as

```Nix
# In home manager config...
programs.direnv = {
  enable = true;
  nix-direnv = {
    enable = true;
    enableFlakes = true;
  };
}
```

To recap, having the snippet above in my home-manager setup, I now can enter any project's root directory and
add a `.envrc` file with the content `use flake ~/src/dotfiles/direnvs/python`.
`~/src/dotfiles/direnvs/python` contains a `flake.nix` (and a `flake.lock`) that has the `devShell` value
I like for all Python projects. When I `cd` into this project, `(nix-)direnv` will read from that `devShell`
and set every tool listed under there. The tools are cached in a `.direnv` directory so when I return here,
the setup is basically instantaneous. Since I make `git` to ignore `.envrc`, and `.direnv` no matter where
they are, this project I'm working on is unaffected by all this.

[direnv]: https://github.com/nix-community/nix-direnv
[nix-direnv]: https://github.com/nix-community/nix-direnv
