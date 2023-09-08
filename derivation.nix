{ rustPlatform, fetchgit, lib, pkgs, ... }:

rustPlatform.buildRustPackage {
    pname = "atai";
    version = "0.1.0";

    src = ./.;

    cargoHash = "";

    buildInputs = with pkgs; [
    ];
}
