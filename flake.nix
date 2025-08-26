{
  description = "Lotus IME flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs ({ inherit system; });
  in
  with pkgs;
  {
    devShells.${system}.default = mkShell {
      buildInputs = [
        cargo
        rustc
      ];
    };

    packages = {
      ${system}.default = callPackage ./default.nix {};
    };
  };
}
