{
  description = "Building my rust project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

      project_name = manifest.name;
      version = manifest.version;

      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      };
      nativeBuildInputs = with pkgs; [
        pkg-config
        gcc
        rust-bin.stable.latest.default
      ];
      buildInputs = with pkgs; [
        # misc
        openssl

        # x11 libs
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11

        # wayland
        wayland

        # GUI libs
        libxkbcommon
        libGL
        fontconfig
      ];
      libraryPath = pkgs.lib.makeLibraryPath buildInputs;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        LD_LIBRARY_PATH = "${libraryPath}:$LD_LIBRARY_PATH";
        inherit buildInputs nativeBuildInputs;
      };
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = project_name;
        inherit version;
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        inherit buildInputs nativeBuildInputs;
        
        postFixup = ''
          patchelf --set-rpath "${libraryPath}" $out/bin/${project_name}
        '';
      };
      apps.x86_64-linux.default = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/${project_name}";
      };
    };
}
