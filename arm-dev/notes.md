# Description
This directory contains the necessary elements to produce an ARM64 emulated machine running NixOS, under QEMU, configured to facilitate development on `rsonpath`.

## Warning
The system's image can be overwritten if the overlay files are modified and `make` is rerun.
This means **the emulated system will be destroyed and rebuilt from scratch**.
For `rsonpath` development, QEMU is configured to share the host system's project root directory with the guest system.
Running `...-init` or `...-reset` can cause data loss for the QEMU-mounted root drive.
Guest system-level modifications should be contained in the system's nixos definition located at [share/etc/nixos/configuration.nix](./share/etc/nixos/configuration.nix).

## Usage
Typical development usage implies the following flow:
1. Have `nix-shell` installed and have it load the `shell.nix` at the root of the project.
2. On first setup, run `just arm-dev-init`.
3. After setup completes, start the guest with `just arm-dev`.
4. Connect to the guest with `just arm-dev-ssh`.
5. `cd /rsonpath && nix-shell`
6. Do `aarch64`-type things.
7. When finished with the guest, use `just arm-dev-shutdown`.

## Commands
The most common uses of the VM have `just` commands which can be run from the project root:
- `just arm-dev-init`
The system will build itself.
This takes about 30 minutes on modern hardware.
The guest will restart once while building.
- `just arm-dev`
After building, the system can be started with this command.
All boot-time prompts (autoboot and NixOS generation selection) can be ignored to allow the system to boot normally.
- `just arm-dev-shutdown`
Gracefully shuts the guest.
- `just arm-dev-kill`
Kills the guest VM ungracefully via `killall armdev-guest`, on the host.
Useful if the device is otherwise unkillable, but `just arm-dev-shutdown` should probably be preferred.
- `just arm-dev-update`
Once the guest is built and running, changes to its configuration can be applied with this command.
Useful to avoid long from-scratch image rebuilds while making changes to the machine config.
- `just arm-dev-tidy`
Cleans up the nix store.
Useful if the guest has filled its drive contents.
May make `nix-shell` slower on next run to repopulate its environment.
- `just arm-dev-reset`
**Destructively** clears the VM contents.
- `just arm-dev-ssh`
Connect to the guest via SSH.
Text interfaces are currently recommended due to the low performance of the GUI.
If there is no need to use GUI, `-nographic` can be added to the stack in [start-vm](start-vm).

## `rsonpath` Development (development)
The guest mounts the project root at `/rsonpath` via 9p.
Changes should be available at both host and guest simultaneously.
In the guest, it may be necessary to invoke `nix-shell` inside project directory to ensure it is setup for development work (This step has also been observed to take a few minutes).

## VM Configuration
QEMU is configured to run the emulated system with the following options:

### SSH (port 22) on the guest is exposed to the host on port 5022.
This enables connecting to the device over ssh using `ssh armdev@localhost -p 5022`.
Public keys can be added to `config.users.users.armdev.openssh.authorizedKeys.keys` in `config/configuration.nix`.
These changes would require an `...-update` to then apply but should not require a restart to take effect.

### A single user, `armdev` 
- is in `wheel`. 
- has a blank password.
- will auto-login in both GUI and serial on boot.
- **will have user home contents be destroyed on a rebuild**.

## Setup Process
The emulated device is built using the following process:
- Download the latest successful `aarch-linux-sd-image` raw file from [Hydra](https://hydra.nixos.org/job/nixos/release-22.11/nixos.sd_image.aarch64-linux).
This image forms the basis for the emulated device's boot drive.
- Downloading the latest `u-boot` file from `nixpkgs` and copying it to the local directory.
- Copying the contents of the [`init`](./init) directory into the image.
- Converting the image to sparse qcow2 and enlarging.
- Boot the machine for first time setup: runs the user scripts provided in the auto-login user profiles and applies the `configuration.nix` in [`config/nixos`](config/nixos).

## Additional Consideration / Future Enhancements
### Use of `nixos-generate`
Using `nixos-generate` would replace the the current `aarch-linux-sd-image.qcow2` target, and streamline the initial boot.
NixOS's `config.system.build.vm` is also a potential option worth additional investigation here.

### Eliminate root
Currently the image creation process requires host root to mount and modify the disk for bootstrapping, but there is probably an unprivileged alternative.

### Link Host and Guest NixStores
Should reduce build times, but permissions and host state may introduce complexity.

### Add `direnv` support
This would eliminate `nix-shell` needing to be explicitly run in the project directory.

### Breakout UID
Permission issues are bound to happen if the user's host UID is not 1000, which is what the guest configures the `armdev` user with.
This could be broken out into a separate `.nix` file and ignored by git to enable a bit more portability for users with alternative setups.

### Simplify `just arm-dev-ssh`
`cd`ing to the project root and running `nix-shell` can probably be added to the command.

### Ensure Correct SIGKILL target
The `just arm-dev-kill` command could be made more unique or use a PID file.