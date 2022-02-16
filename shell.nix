{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    packages = with pkgs; [
      rustc
      cargo
      rust-analyzer
      gcc
      pkg-config
      clippy
      openssl
      clang
      cmake
    ];
  }
