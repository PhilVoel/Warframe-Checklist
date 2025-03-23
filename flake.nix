{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.11";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}";
			in {
				devShells.default = pkgs.mkShell {
					name = "WF Checklist";
					packages = with pkgs; [
						bacon
						cargo
					];
				};
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "WF Checklist";
					version = "0.1.0";
					src = self;
					cargoLock.lockFile = ./Cargo.lock;
				};
			}
		);
}
