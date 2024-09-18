{ ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "d";
      inputsFrom = [
        self'.devShells.rust
        config.treefmt.build.devShell
      ];
      packages = with pkgs; [
        cargo-watch
        config.process-compose.cargo-doc-live.outputs.package
	just
      ];
    };
  };
}
