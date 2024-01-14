{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import ./overlay.nix)
        ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
      in rec {
        packages = rec {
          demostf-backup = pkgs.demostf-backup;
          dockerImage = pkgs.callPackage ./docker.nix {};
          default = demostf-backup;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo rustfmt clippy cargo-edit cargo-audit bacon];
        };
      }
    )
    // {
      overlays.default = import ./overlay.nix;
      nixosModules.default = {
        pkgs,
        config,
        lib,
        ...
      }: {
        imports = [./module.nix];
        config = lib.mkIf config.services.demostf-backup.enable {
          nixpkgs.overlays = [self.overlays.default];
          services.demostf-backup.package = lib.mkDefault pkgs.demostf-backup;
        };
      };
    };
}
