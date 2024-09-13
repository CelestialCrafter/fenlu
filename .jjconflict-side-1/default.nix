{ fetchFromGitHub, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "fenlu";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "CelestialCrafter";
    repo = "fenlu";
    rev = "master";
    sha256 = "sha256-jh5rhmoy7hZjvMuejmzKgaOwYO8EJAghVZDAL06Yn9c=";
  };

  cargoLock.lockFile = ./Cargo.lock;
}
