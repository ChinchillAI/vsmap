{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs =
    { self, nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      macroquad-deps = with pkgs; [
        libX11
        libxi
        alsa-lib
        libxkbcommon
        libGL

        lld
        binaryen
      ];
    in
    {
      packages.x86_64-linux = {
        vsmap-gui = pkgs.rustPlatform.buildRustPackage {
          pname = "vsmap-gui";
          version = "0.1.0";

          src = ./.;
          buildInputs = macroquad-deps;

          buildPhase = ''
            cargo build -p vsmap-web --release
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/vsmap-web $out/bin/vsmap-gui
          '';

          cargoLock.lockFile = ./Cargo.lock;
        };
        vsmap-cli = pkgs.rustPlatform.buildRustPackage {
          pname = "vsmap";
          version = "0.1.0";

          src = ./.;

          buildAndTestSubdir = "crates/cli";
          cargoLock.lockFile = ./Cargo.lock;
        };

        vsmap-web = pkgs.rustPlatform.buildRustPackage {
          pname = "vsmap-web";
          version = "0.1.0";

          src = ./.;
          nativeBuildInputs = with pkgs; [
            lld
            binaryen
          ];
          buildInputs = macroquad-deps;

          cargoLock.lockFile = ./Cargo.lock;

          buildPhase = ''
            cargo xtask build
          '';

          installPhase = ''
            cp -r build/ $out
          '';
        };
      };

      nixosModules.default =
        {
          lib,
          config,
          pkgs,
          ...
        }:
        let
          cfg = config.services.vsmap;
        in
        {
          options.services.vsmap.enable = lib.mkEnableOption "Enables VSMap WASM server";

          config = lib.mkIf cfg.enable {
            services.nginx.virtualHosts."vsmap" = {
              root = "${self.packages.x86_64-linux.vsmap-web}";
              enableACME = true;
            };
          };
        };

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages =
          with pkgs;
          [
            rustc
            cargo
            rustfmt
          ]
          ++ macroquad-deps;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath macroquad-deps;
      };
    };
}
