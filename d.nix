{ stdenv }:

let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  phases = [ "unpackPhase" "installPhase" ];

  installPhase = ''
    mkdir -p $out/d
    cp -r $src/* $out/d
    install -D $src/bin/d $out/bin/d
  '';

  postFixup = ''
    wrapProgram $out/bin/d --prefix PATH : $out/bin
  '';
}
