{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs = {
        systems.follows = "systems";
      };
    };
  };

  outputs = {
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "arbejdstid";
          version = "0.0.1";

          cargoHash = "sha256-OcbYThSp4j+0BaIten1fSR26dHG04V27svjMIEZVLbo=";

          src = ./.;
        };
      }
    );
}
