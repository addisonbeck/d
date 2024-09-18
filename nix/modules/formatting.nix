{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    treefmt.config = {
      projectRootFile = "flake.nix";
      programs = {
        alejandra.enable = true;
        rustfmt.enable = true;
      };
    };
  };
}
