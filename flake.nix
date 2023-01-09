{
  inputs = {
    nixpkgs = {
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };

    rust-overlay = {
      type = "github";
      owner = "oxalica";
      repo = "rust-overlay";
    };

    flake-utils = {
      type = "github";
      owner = "numtide";
      repo = "flake-utils";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [(import rust-overlay)];
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.nightly."2023-01-08".default.override {
              extensions = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-src"
              ];
            })
          ];
        };
      }
    );
}

