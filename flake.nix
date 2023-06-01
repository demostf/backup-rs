{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        lib = pkgs.lib;
      in
        rec {
          # `nix build`
          packages = rec {
            demobackup = naersk-lib.buildPackage {
              pname = "demobackup";
              root = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "src" "src/.*"];
            };
            dockerImage = pkgs.dockerTools.buildImage {
              name = "demostf/backup";
              tag = "latest";
              copyToRoot = [demobackup];
              config = {
                Cmd = [ "${demobackup}/bin/backup"];
              };
            };
          };
          defaultPackage = packages.demobackup;

          # `nix run`
          apps.hello-world = flake-utils.lib.mkApp {
            drv = packages.demobackup;
          };
          defaultApp = apps.demobackup;

          # `nix develop`
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    )
    // {
      nixosModule = {
        config,
        lib,
        pkgs,
        ...
      }:
        with lib; let
          cfg = config.services.demosbackup;
        in {
          options.services.demosbackup = {
            enable = mkEnableOption "Enables the demos backup service";

            target = mkOption {
              type = types.str;
              description = "target directory";
            };
            api = mkOption {
              type = types.str;
              default = "https://api.demos.tf";
              description = "demos.tf api url";
            };
            stateFile = mkOption {
              type = types.str;
              description = "state file path";
            };
            logLevel = mkOption {
              type = types.str;
              default = "INFO";
              description = "log level";
            };
            user = mkOption {
              type = types.str;
              description = "user that owns the demos";
            };
            interval = mkOption {
              type = types.str;
              default = "*:0/10";
              description = "Interval to run the service";
            };
          };

          config = mkIf cfg.enable {
            systemd.services.demosbackup = let
              pkg = self.defaultPackage.${pkgs.system};
            in {
              script = "${pkg}/bin/backup";
              description = "Backup demos for demos.tf";

              environment = {
                STORAGE_ROOT = cfg.target;
                SOURCE = cfg.api;
                STATE_FILE = cfg.stateFile;
                RUST_LOG = cfg.logLevel;
              };

              serviceConfig = {
                ReadWritePaths = [cfg.target cfg.stateFile];
                Restart = "on-failure";
                User = cfg.user;
                PrivateTmp = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                NoNewPrivileges = true;
                PrivateDevices = true;
                ProtectClock = true;
                CapabilityBoundingSet = true;
                ProtectKernelLogs = true;
                ProtectControlGroups = true;
                SystemCallArchitectures = "native";
                ProtectKernelModules = true;
                RestrictNamespaces = true;
                MemoryDenyWriteExecute = true;
                ProtectHostname = true;
                LockPersonality = true;
                ProtectKernelTunables = true;
                RestrictAddressFamilies = "AF_INET AF_INET6";
                RestrictRealtime = true;
                ProtectProc = "noaccess";
                SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
                IPAddressDeny = "localhost link-local multicast";
              };
            };

            systemd.timers.demosbackup = {
              enable = true;
              description = "Backup demos for demos.tf";
              wantedBy = ["multi-user.target"];
              timerConfig = {
                OnCalendar = "*:0/10";
              };
            };
          };
        };
    };
}
