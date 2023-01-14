{
  inputs = {
    nixpkgs = {
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };

    fenix = {
      type = "github";
      owner = "nix-community";
      repo = "fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      type = "github";
      owner = "ipetkov";
      repo = "crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils = {
      type = "github";
      owner = "numtide";
      repo = "flake-utils";
    };
  };

  outputs = {
    nixpkgs,
    fenix,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        rustToolchain = fenix.packages.${system}.complete;

        craneLib = (crane.lib.${system}.overrideToolchain
          (rustToolchain.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
          ]));

        src = ./.;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (rustToolchain.withComponents [
              "rustc"
              "cargo"
              "clippy"
              "rustfmt"
              "rust-src"
            ])
          ];
        };

        checks = {
          rustfmt = craneLib.cargoFmt {
            inherit src;
          };

          obce-substrate-std = craneLib.cargoTest {
            inherit src;
            cargoArtifacts = null;
            cargoExtraArgs = "--features substrate-std";
          };

          obce-ink-std = craneLib.cargoTest {
            inherit src;
            cargoArtifacts = null;
            cargoExtraArgs = "--features ink-std";
          };
        };
      }
    );
}
