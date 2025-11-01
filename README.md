# SpeedCam

A vehicle speed camera based on a Raspberry Pi 4, a Canon DSLR and cheap radar modules.

## Development (Nix)

This repository includes a `flake.nix` and uses Nix to provide a reproducible development
environment. To get a shell with all required development tools run:

```bash
# enter the flake's development shell (requires Nix with flakes enabled)
nix develop
```

Once inside the Nix shell you have the tools available to build the project, for example:

```bash
cd speedcam
cargo build
```

If you are developing on a system without Nix installed, you can also build and test the project
using standard Rust tooling, provided you have Rust and Cargo installed. The `flake.nix`file should
give you an idea of the required dependencies.
