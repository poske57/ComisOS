{
  description = "ComisOS development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer" ];
        };
        comis = pkgs.rustPlatform.buildRustPackage {
          pname = "comis";
          version = "0.1.0";
          src = ./comis;
          cargoLock.lockFile = ./comis/Cargo.lock;
        };
      in
      {
        packages.default = comis;
        apps.default = {
          type = "app";
          program = "${comis}/bin/comis";
        };
        devShells.default = pkgs.mkShell {
          packages = [ toolchain ];
        };
      });
}
