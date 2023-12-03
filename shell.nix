{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ ];
  buildInputs = with pkgs; [
    rustfmt
    clippy
    cargo-outdated
    cargo-udeps
    cargo-edit
    rustc
    cargo
    rust-analyzer
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
