with import <nixpkgs> {};

let
    # stable = <nixos-22.05> {};
    stable = import (fetchTarball https://nixos.org/channels/nixos-22.05/nixexprs.tar.xz) {};
in stable.clangStdenv.mkDerivation {
  name = "clang-nix-shell";
  buildInputs = [ ];
}
