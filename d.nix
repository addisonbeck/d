{ stdenv }:

let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  phases = [ "unpackPhase" "installPhase" ];

  installPhase = ''
    mkdir -p $out/bin
    cp -r $src/lib $out/
    install -D $src/bin/d $out/bin
  '';
}
