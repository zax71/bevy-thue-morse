{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              # Rust dependencies
              (rust-bin.beta.latest.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })
              pkg-config
            ]
            ++ lib.optionals (lib.strings.hasInfix "linux" system) [
              # for Linux
              # Audio (Linux only)
              alsa-lib
              # Cross Platform 3D Graphics API
              vulkan-loader
              # For debugging around vulkan
              vulkan-tools
              # Other dependencies
              libudev-zero
              libx11
              libxcursor
              libxi
              libxrandr
              libxkbcommon
              wayland
              pkg-config

              # Utility that builds rust-generated WebAssembly package
              wasm-pack
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              libx11
              libxi
              libxcursor
              libxkbcommon
              wayland
            ];
          };
      }
    );
}
