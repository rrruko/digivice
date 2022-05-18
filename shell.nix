let
  nixpkgs =
    import (builtins.fetchTarball {
      name = "nixos-21.11";
      url =
      "https://github.com/nixos/nixpkgs/archive/fd43ce017d4c95f47166d28664a004f57458a0b1.tar.gz";
      sha256 = "1lkc5nbl53wlqp9hv03jlh7yyqn02933xdii5q7kbn49rplrx619";
    }) {};
in
  nixpkgs.mkShell {
    nativeBuildInputs = [
      nixpkgs.cargo
      nixpkgs.rustc
      nixpkgs.elmPackages.elm
    ];
  }
