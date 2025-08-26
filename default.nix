{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
, fetchFromGitHub ? pkgs.fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "lotus-ime";
  version = "1.0-unstable";

  src = ./.;

  buildInputs = [];
  nativeBuildInputs = [];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    homepage = "";
    description = "lotus-ime rust";
    license = licenses.mit;
  };
}