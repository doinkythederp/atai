{
  inputs = { flake-utils.url = "github:numtide/flake-utils"; };

  outputs = { self, nixpkgs, flake-utils, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        devShells.${system} = import ./shell.nix;

        packages = rec {
          atai = pkgs.callPackage ./derivation.nix { };
          default = atai;
        };
        apps = rec {
          atai = flake-utils.lib.mkApp { drv = packages.atai; };
          default = atai;
        };
      }));
}
