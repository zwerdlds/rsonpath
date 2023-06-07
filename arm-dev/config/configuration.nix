{ config, pkgs, lib, ... }:
{
  imports =
    [
      # Use the NixOS-generated hardware configuration.
      "/etc/nixos/hardware-configuration.nix"

      # Set up the appropriate guest kernel PCI drivers etc.
      <nixpkgs/nixos/modules/profiles/qemu-guest.nix>
    ];

  config.services.sshd.enable = true;

  config.environment.systemPackages = with pkgs; [
    wget
    direnv
    sysbench
  ];

  # Start an SSH daemon at boot.
  config.services.openssh.enable = true;

  # Expose SSH ports
  config.networking.firewall.allowedTCPPorts = [
    22
  ];

  # NixOS wants to enable GRUB by default, which is not compatible with this system
  config.boot.loader.grub.enable = false;

  # Talk to the host QEMU instance
  config.services.qemuGuest.enable = true;

  # Generates /boot/extlinux/extlinux.conf for machine bootloader
  config.boot.loader.generic-extlinux-compatible = {
    enable = true;
    configurationLimit = 2;
  };

  config.nix.settings = {
    experimental-features = [ "nix-command" "flakes" ];
    auto-optimise-store = true;
  };

  # Autologin
  config.services.getty.autologinUser = "armdev";

  # Define the single user for this system
  config.users.users.armdev = {
    isNormalUser = true;
    home = "/home/armdev";
    extraGroups = [ "wheel" ];
    shell = pkgs.nushell;
    uid = 1000;
    password = "";
    openssh.authorizedKeys.keys = [
      "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDgH1sFlkb9Cs3ub5P8cwzMxBgZfSFWQN7gZZLPnGDHEGLkkip5xrxzKJDfiGHNsEewN5WnmfHGi82ihfKzlZyOow3elr/udnCq9aNMR5wpa5nvS6xQh4grICMuHg7N4RPjSLSZoEFG4vRSVqkT0+f76+Qd8Q8hdLw6vmlG1DW5JaDsdwmpa/Bh3JCsp7RASqa995/2HGlHo+bzE/W3LaQVNAiPOt6LzYpq4/lNQiFyZ0A6cxKbJcSa6z7a/fdTGEL8A3s1Axmku/ZJ9B6lXgp7fFcz4UKArxsbNToneRlrmuUVY8wJ65qFDb3IgWpmKt0O1ijwRJdRYLSltJ/vXJPzF8hN8lKflWpOgjcZMUZ24gpwtYJE9rL1peLHhhU/q18wq3/GzAQEp1jAisF2sFDKtqyKQ/djhvhB+Q+BcJHlkpbpCvw2fP5tyCts7tktCQCAus+8RQsTSPIYxw7DqaqTCvVZ3IX19ujjGf8rGwf0Mzx02SraWhIxzhDl1XYKGp6AXOtA9QQuToC3Zdkn9N9NdQQj9QAwSGAOE/Mo5u13xP+24AWkxI0/64u1wj4KslyLLH4x8gT06Xc1BxSorfnB900l8U4pedPUST+5OfyfUEX/G/radX/Vwj9aUJ1jIqQUtg5V+y17RICeLwN0qp30hHSwzSY3R7yHunkciFeUtQ== zwerdlds@sulla"
    ];
  };

  config.fileSystems = {
    "/" = {
      device = "/dev/vda2";
      fsType = "ext4";
    };
    "/rsonpath" = {
      device = "rsonpath";
      fsType = "9p";
    };
    "/etc/nixos" = {
      device = "nixconfig";
      fsType = "9p";
    };
  };

  config.system.stateVersion = "22.11";
}
