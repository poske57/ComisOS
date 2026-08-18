{ config, pkgs, ... }:

{
  home.username = "%username%";
  home.homeDirectory = "/home/" + config.home.username;

  home.stateVersion = "26.05";

  home.packages = with pkgs;[
  ];

  programs = {
    git = {
      enable = true;
    };
    firefox = {
      enable = true;
    };
    alacritty = {
      enable = true;
    };
    opencode = {
      enable = true;
    };
  };

  services = {
  };

  xdg.enable = true;

  home.sessionVariables = {
  };

  home.file = {
  };

  # Let Home Manager install and manage itself.
  programs.home-manager.enable = true;
}
