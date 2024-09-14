{ stdenv, pkgs }:

let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  nativeBuildInputs = [ ];
  buildInputs = [ pkgs.makeWrapper ];

  buildPhase = "";

  installPhase = ''
    mkdir -p $out/d/
    cp -r $src $out/d
    install -D $src/bin/d $out/d/bin/d
  '';

  postFixup = ''
     wrapProgram $out/d/bin/d --prefix PATH : $out/bin
  '';
}
