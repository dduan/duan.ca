{
  description = "Static site generator for duan.ca";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      rec {
        packages = flake-utils.lib.flattenTree
          {
            generator = rustPlatform.buildRustPackage rec {
              pname = "duan.ca-generator";
              version = "0.1.0";
              src = ./.;
              cargoSha256 = "sha256-dIbynlmQ+wx1UzcyMv/yQfwkzQwfu/bovP55t+CIYKk=";
              lockFile = ./Cargo.lock;
              LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
            };
          };
        defaultPackage = packages.generator;
        devShell = pkgs.mkShell {
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          buildInputs = [
            cargo
            rust-analyzer
            rustc
            libclang
          ];
        };
      });
}
