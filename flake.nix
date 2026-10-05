{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, utils, naersk, nixpkgs-mozilla }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        toolchain = (pkgs.rustChannelOf {
          channel = "nightly";
          date = "2026-10-05";
          sha256 = "sha256-2XBkcfgQpF5hzNejCfabTCqchftapwoyjmHsCKsQ6pc=";
        }).rust;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        packages.default = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];
          buildInputs = with pkgs; [ openssl ];
          postInstall = ''
            wrapProgram $out/bin/bandrip \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}
          '';
        };
        devShells.default = with pkgs; mkShell {
          buildInputs = [ toolchain rustfmt pre-commit rustPackages.clippy pkg-config openssl ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
