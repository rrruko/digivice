{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.digivice = naersk-lib.buildPackage {
            pname = "digivice";
            root = ./.;
          };
          packages.default = packages.digivice;

          packages.frontend = pkgs.stdenv.mkDerivation {
            name = "frontend";
            src = ./frontend;
            installPhase = ''
              cp -r $src $out
            '';
          };

          packages.digimons = pkgs.stdenv.mkDerivation {
            name = "digimons";
            src = ./digimons;
            installPhase = ''
              cp -r $src $out
            '';
          };

          # `nix run`
          apps.digivice = flake-utils.lib.mkApp {
            drv = packages.digivice;
          };
          defaultApp = apps.digivice;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo elmPackages.elm ];
          };
        }
    );
}
