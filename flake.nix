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
        packages = rec {
          default = warframe-checklist;
          warframe-checklist = pkgs.rustPlatform.buildRustPackage {
            pname = "wf_checklist";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs = with pkgs; [
              lua5_4
            ];

			postPatch = ''
			  substituteInPlace src/main.rs --replace "curl" "${pkgs.curl}/bin/curl"
			'';

            postInstall = ''
              mkdir -p $out/share/wf-checklist
              cp -r frontend $out/share/wf-checklist/
            '';
          };
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
        in
        {
          options.services.warframe-checklist = {
            enable = lib.mkEnableOption "Warframe Checklist";
            user = lib.mkOption {
              type = lib.types.str;
              default = "wf-checklist";
            };
            port = lib.mkOption {
              type = lib.types.int;
              default = 4000;
              description = "Port to listen on";
            };
            stateDir = lib.mkOption {
              type = lib.types.str;
              default = "/var/lib/wf-checklist/";
              description = "Directory for state files";
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.${cfg.user} = {
              isSystemUser = true;
              group = cfg.user;
            };
            users.groups.${cfg.user} = { };

            systemd.tmpfiles.rules = [ "d ${cfg.stateDir} 0750 ${cfg.user} ${cfg.user} -" ];
            systemd.services.warframe-checklist = {
              description = "Warframe Checklist";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];

              serviceConfig = {
                ExecStart = "${self.packages.${pkgs.system}.warframe-checklist}/bin/wf_checklist";
                Restart = "always";
                User = cfg.user;
                Group = cfg.user;
                WorkingDirectory = "${self.packages.${pkgs.system}.warframe-checklist}/share/wf-checklist";
                Environment = ''
                  ROCKET_PORT=${toString cfg.port} \
                  WF_STATE_DIR=${cfg.stateDir}
                '';
              };
            };
          };
        };
    };
}
