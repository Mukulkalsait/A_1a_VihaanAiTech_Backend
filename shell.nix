{ pkgs ? import <nixpkgs> { } }:

let
  # NEW VERSION PER BUILD ✨
  # fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
  # rustToolchain = fenix.complete.toolchain;


  # PINNED VERSION: ✨
  fenix = import
    (fetchTarball {
      url = "https://github.com/nix-community/fenix/archive/main.tar.gz";
      sha256 = "sha256-xxxxxxxxxxxxxxxx";
    })
    { };

in

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustToolchain

    openssl
    pkg-config
    clang
    llvmPackages.lld
    # binutils # only when debuging linker Issues.

    sqlite
    sqlx-cli
  ];

  OPENSSL_NO_VENDOR = 1;

  shellHook = ''

    export HISTFILE="$PWD/.zsh_history"
    export HISTSIZE=10000
    export SAVEHIST=10000

    # Toolchain
    export CC=clang
    export CXX=clang++
    export LD=ld.lld
    export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"

    # 🔑 CRITICAL RING FIX
    export RING_PREGENERATE_ASM=1

    echo "🟢 Nix shell ready (ring fixed)"
    if [ -z "$ZSH_VERSION" ]; then
      zsh
      exit
    fi



    echo "🦀 Rust backend dev shell"
    rustc --version
    cargo --version
  '';
}
