{
    description = "A nix flake for development of the CIA speed camera project";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs { inherit system overlays; };
        in
        {
            devShells.default = pkgs.mkShell {
                buildInputs = with pkgs; [
                    git
                    pkg-config
                    libgphoto2
                    clang
                    llvmPackages.libclang
                    ( rust-bin.stable.latest.default.override { extensions = ["rust-src"]; } )
                    ansible
                ];
            };

            packages = {
                default = self.packages.${system}.speedcam;

                speedcam = pkgs.rustPlatform.buildRustPackage {
                    pname = "speedcam";
                    version = "0.1.0";

                    src = ./speedcam;

                    buildInputs = [ pkgs.libgphoto2 pkgs.libudev-zero ];
                    nativeBuildInputs = with pkgs; [ pkg-config clang rustPlatform.bindgenHook ];
                    cargoHash = "sha256-OEecRCBLLaSjNoaYFHuJEVS12mQawnTvHvPUBdNLsH4=";
                    doCheck = false;
                };
            };
        }
    );
}

