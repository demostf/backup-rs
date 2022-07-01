{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.demobackup = naersk-lib.buildPackage {
            pname = "demobackup";
            root = ./.;
          };
          defaultPackage = packages.demobackup;

          # `nix run`
          apps.hello-world = flake-utils.lib.mkApp {
            drv = packages.demobackup;
          };
          defaultApp = apps.demobackup;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
