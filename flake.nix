{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override
          {
            targets = [ "wasm32-unknown-emscripten" ];
          };
      in
      {
        devShells.default = pkgs.mkShell {
          EM_CACHE = "/tmp/emscripten_cache";
          packages = with pkgs; [
            bashInteractive

            # rust
            emscripten
            rust

            # node
            nodejs

            # svelte
            nodePackages.svelte-language-server
          ];
          shellHook = ''
            export CC="${pkgs.emscripten}/bin/emcc"
          '';
        };
      });
}

