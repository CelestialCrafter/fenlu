{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [ gcc-unwrapped.lib ];
        packages = with pkgs; [
          qt6.full
          cargo
          rustc
          cargo-watch
          pkg-config

          python312Packages.pillow
        ];
      };
    };
}
