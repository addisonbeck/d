{
  description = "A collection of system automations";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      systems =
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    in {
      packages = builtins.listToAttrs (builtins.map (system:
        {
          name = system;
          value = rec {
            d = nixpkgs.legacyPackages.${system}.callPackage ./d.nix {};
            default = d;
          };
        }) systems);
    };
}
