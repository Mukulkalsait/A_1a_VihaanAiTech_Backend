{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
  ];

  OPENSSL_NO_VENDOR = 1;

  shellHook = ''
    export HISTFILE="$PWD/.zsh_history"
    export HISTSIZE=10000
    export SAVEHIST=10000
    echo "📜 Using project-local shell history"
      echo "🟢 Entered Nix shell"
      if [ -z "$ZSH_VERSION" ]; then
        zsh
        exit
      fi
  '';

}
