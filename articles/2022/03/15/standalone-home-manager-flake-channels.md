# Flake, Home Manager, and Extra Packages
2022-03-15T18:53:59-07:00
tag: Nix, Nixpkgs, home-manager

So, you use a standalone home-manager, it's set up with flake, tracking a particular nixpkgs channel. How do
you use a package from another channel? This seemingly simple task took me, a Nix noob, quite a bit of
research to solve. Here's how I did it.

A simple flake.nix for home-manager might look like this:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    home-manager.url = "github:nix-community/home-manager/release-21.11";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, home-manager }: {
    "dan@some-mac" = home-manager.lib.homeManagerConfiguration {
      username = "dan";
      system = "x86_64-darwin";
      homeDirectory = "/home/dan";
      configuration = { config, pkgs ... }: {
          home.packages = [
            pkgs.hello
          ];
      };
    };
  };
}
```

A few things of note:

1. home manager is set to follow `nixpkgs`, which tracks `nixos-21.11`.
2. `pkgs.hello` refers to the package in `nixos-21.11` as well.

To put things in concrete terms, our goal is to put a package from a channel other than `nixos-21.11`
alongside `pkgs.hello`.

The key to my solution is the `extraModules` in `home-manager.lib.homeManagerConfiguration`'s argument set.
We'll leverage it to modify the environment made available to its sibling, `configuration`.

First, add the new channel as input `nixpkgs-unstable`:

```nix
{
  inputs = {
    # ...
    nixpkgs-unstable.url = "github:nixos/nixpkgs";
  };
  ...
}
```

Then, add a small module to `extraModules`. In it we make `nixpkgs-unstable` an argument to `_module`.

```nix
{
  input = { ... };
  outputs = { self, nixpkgs, nixpkgs-unstable, home-manager }: { # Note we also pass in nixpkgs-unstable here
    "dan@some-mac" = home-manager.lib.homeManagerConfiguration {
      extraModules = [
        ({ pkgs, ... }: rec {
          _module.args.nixpkgs-unstable = import nixpkgs-unstable { inherit system; };
        })
      ];
      # ...
    };
  };
}
```

... now that we added `args.nixpkgs-unstable`, it becomes available in the configuration:

```nix
{
  input = { ... };
  outputs = { self, nixpkgs, nixpkgs-unstable, home-manager }: { # Note we also pass in nixpkgs-unstable here
    "dan@some-mac" = home-manager.lib.homeManagerConfiguration {
      # ...
      configuration = { config, pkgs, nixpkgs-unstable, ... }:
          home.packages = [
            pkgs.hello
            nixpkgs-unstable.python39Packages.python-lsp-server
          ];
      };
    };
  };
}
```

There, we made `python39Packages.python-lsp-server` from nixpkgs's master branch appear alongside our
standard, default channel.

And that's how you add packages from a different channel in a flake setup for standalone home-manager.
