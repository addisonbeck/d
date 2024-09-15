{ stdenv }:

let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  phases = [ "unpackPhase" "installPhase" ];

  installPhase = ''
    mkdir -p $out/d
    cp -r $src/* $out/d
  '';

  postFixup = ''
    wrapProgram $out/d/bin/d --prefix PATH : $out/d/bin/d
  '';
}
