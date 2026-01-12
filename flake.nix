{
  description = "Warframe Checklist application and service";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils/v1.0.0";
  };

  outputs =
    {
      flake-utils,
      nixpkgs,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
      in
      {
        devShells.default = pkgs.mkShell {
          name = "WF Checklist";
          packages = with pkgs; [
            bacon
            cargo
            clippy
            pkg-config
            lua5_4
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.lua5_4 ];
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "WF Checklist";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    )
    // {
      nixosModules.warframe-checklist =
        {
          config,
          pkgs,
          lib,
          ...
        }:
        let
          cfg = config.services.warframe-checklist;
          stateDir = "/var/lib/wf-cheklist";
        in
        {
          options.services.warframe-checklist = {
            enable = lib.mkEnableOption "Warframe Checklist";
            user = lib.mkOption {
              type = lib.types.str;
              default = "wf-cheklist";
            };
            port = lib.mkOption {
              type = lib.types.int;
              default = 4000;
              description = "Port to listen on";
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.${cfg.user} = {
              isSystemUser = true;
              group = cfg.user;
            };
            users.groups.${cfg.user} = { };

			systemd.tmpfiles.rules = [ "d ${stateDir} 0750 ${cfg.user} ${cfg.user} -" ];
            systemd.services.warframe-checklist = {
              description = "Warframe Checklist";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];


              serviceConfig = {
                ExecStart = "${self.packages.${pkgs.system}}/bin/wf_checklist";
                Restart = "always";
                User = cfg.user;
                Group = cfg.group;
                WorkingDirectory = stateDir;
                Environment = "ROCKET_PORT=${toString cfg.port}";
              };
            };
          };
        };
    };
}
