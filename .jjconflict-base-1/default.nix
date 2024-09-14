{ rustPlatform, full }:
rustPlatform.buildRustPackage {
  pname = "fenlu";
  version = "0.1.0";
  src = ./.;
  nativeBuildInputs = [ full ];
  cargoLock.lockFile = ./Cargo.lock;
}
