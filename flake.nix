{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          go

          (python3.withPackages (python-pkgs: [
            python-pkgs.python
            python-pkgs.pillow
            python-pkgs.requests
          ]))
        ];
      };
    };
}

