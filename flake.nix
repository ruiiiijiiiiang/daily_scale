{
  description = "Daily Scale Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        dailyScalePkg = pkgs.rustPlatform.buildRustPackage {
          pname = "daily_scale";
          version = "0.1.2";

          src = ./.;

          cargoHash = "sha256-wvnXuby2tMcLGLWIOq7d0raikHD91VcvzsIWTawM1ow=";
        };
      in
      {
        devShell = pkgs.mkShell {
          packages = [
            pkgs.rust-bin.stable.latest.default
          ];
        };

        packages.default = dailyScalePkg;

        apps.default = {
          type = "app";
          program = "${dailyScalePkg}/bin/daily_scale";
        };
      }
    );
}
