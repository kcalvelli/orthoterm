# This flake was initially generated by fh, the CLI for FlakeHub (version 0.1.18)
{
  # A helpful description of your flake
  description = "Liturgical Calendar Service";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # Flake outputs that other flakes can use
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
        ];
        
        buildInputs = with pkgs; [
          openssl
          rust-analyzer
          cargo-watch
          cargo-edit
        ];

        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          packages = [
            toolchain
          ];

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = 1;
          RUST_LOG = "debug";
          
          # Required for reqwest/openssl
          OPENSSL_NO_VENDOR = 1;
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        };
      }
    );
}
