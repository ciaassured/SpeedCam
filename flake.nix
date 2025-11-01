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
                    ansible
                    libgphoto2
                    rust-bin.stable.latest.default
                ];
            };
        }
    );
}