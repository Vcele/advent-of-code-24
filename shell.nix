{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
  ];
}
