{ stdenv, pkgs, ... }:
let version = "0.0.1";
in stdenv.mkDerivation {

  name = "d-${version}";

  src = ./src;

  buildInputs = [ pkgs.makeWrapper ];
  phases = [ "unpackPhase" "installPhase" "postInstall" ];

  installPhase = ''
    mkdir -p $out/d
    cp -r $src/* $out/d
  '';

  postInstall = ''
    mkdir -p $out/bin
    ln -fs "$out/d/bin/d" $out/bin/d
  '';
}
