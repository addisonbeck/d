{ stdenv, pkgs }:

let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  nativeBuildInputs = [ ];
  buildInputs = [ pkgs.makeWrapper ];

  buildPhase = "";

  installPhase = ''
    cp -r $src $out
  '';

  postFixup = ''
     wrapProgram $out/bin/d --prefix PATH : $out/bin
  '';
}
