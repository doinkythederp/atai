{ rustPlatform, fetchgit, lib, pkgs, ... }:

rustPlatform.buildRustPackage {
  pname = "atai";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-baogO0MT1fDCd9f1y3H6QqVJQQxPoQcOx/9NnfkjzsE=";

  buildInputs = with pkgs; [ ];

  doCheck = false;
}
