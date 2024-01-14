{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.demostf-backup;
in {
  options.services.demostf-backup = {
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

    package = mkOption {
      type = types.package;
      defaultText = literalExpression "pkgs.demostf-backup";
      description = "package to use";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.demostf-backup = {
      description = "Backup demos for demos.tf";

      environment = {
        STORAGE_ROOT = cfg.target;
        SOURCE = cfg.api;
        STATE_FILE = cfg.stateFile;
        RUST_LOG = cfg.logLevel;
      };

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/demostf-backup";
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

    systemd.timers.demostf-backup = {
      enable = true;
      description = "Backup demos for demos.tf";
      wantedBy = ["multi-user.target"];
      timerConfig = {
        OnCalendar = "*:0/10";
      };
    };
  };
}
