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

## Hardware Setup

This project is based around the following core components:
- Raspberry Pi 4B
- Canon Rebel T2i DSLR camera
- HLK-LD2451 FMCW ranging radar module
- HLK-LD2415H Doppler speed radar module

The camera is connected via USB to the Raspberry Pi, and the radar modules are connected via UART GPIO pins.

TODO: Add hardware setup instructions here, including wiring diagrams, power requirements, mounting suggestions, etc.

## Software Deployment (Ansible)

To deploy the speed camera software on a Raspberry Pi, you can use the provided Ansible playbook. Make sure you have Ansible installed on your local machine (included in the Nix flake), and then run the following command from the project root:

```bash
ansible-playbook playbooks/setup.yaml
```

This will configure the Raspberry Pi with all necessary dependencies and install the speed camera software as a systemd service.

The Ansible playbook assumes that the target Raspberry Pi is accessible via SSH as `speedcam` with the default install of Raspberry Pi OS Lite. You may need to adjust the inventory file or playbook variables to match your setup.

## Operation

TODO: Add instructions on how to operate the speed camera once deployed (angle of radar, etc.).
TODO: Add information about operating principles (e.g. double radar operation, camera triggering, etc.).