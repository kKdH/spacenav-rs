{
  description = "A Nix-flake-based development environment for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
        );
        buildInputs = with pkgs; [
          expat
          fontconfig
          freetype
          freetype.dev
          libGL
          pkg-config
          libx11
          libxcursor
          libxi
          libxrandr
          wayland
          libxkbcommon
        ];
      in
      {
        devShells.default = craneLib.devShell {
          inherit buildInputs;
          packages = with pkgs; [
            bacon
            cargo-nextest
            clippy
            gcc
            libclang
          ];
          LIBCLANG_PATH = "${pkgs.lib.getLib pkgs.libclang}/lib";
          LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
