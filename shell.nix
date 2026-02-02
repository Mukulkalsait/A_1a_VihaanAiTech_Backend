{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.clang
    pkgs.llvmPackages.lld
    pkgs.binutils
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
  '';
}
