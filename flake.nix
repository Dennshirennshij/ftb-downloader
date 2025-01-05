{
  description = "Building my rust project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachSystem [
      # Supported systems
      "x86_64-linux"
    ] (system: let

      desktop_name = "FTB Downloader";

      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

      project_name = manifest.name;
      version = manifest.version;

      pkgs = import nixpkgs {
        inherit system;
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
      desktopEntry = pkgs.makeDesktopItem {
        name = "${project_name}";
        desktopName = desktop_name;
        exec = "ftb-downloader";
        icon = "";
        comment = "";
        categories = [];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = "${libraryPath}:$LD_LIBRARY_PATH";
        inherit buildInputs nativeBuildInputs;
      };
      packages.from_source = pkgs.rustPlatform.buildRustPackage {
        pname = project_name;
        inherit version;
        cargoLock.lockFile = ./Cargo.lock;
        src = ./.;
        inherit buildInputs nativeBuildInputs;
        
        postFixup = ''
          patchelf --set-rpath "${libraryPath}" $out/bin/${project_name}
        '';
        desktopItems = [ desktopEntry ];
      };
      packages.default = 
        let
          bin = pkgs.fetchurl {
            #url = "https://github.com/Dennshirennshij/${project_name}/releases/download/v${version}/${project_name}-v${version}-${system}";
            #url = "https://github.com/Dennshirennshij/Hello-World/releases/download/v1.0.0/Hello-World-v1.0.0-x86_64-linux";
            url = "https://github.com/Dennshirennshij/ftb-downloader/releases/download/v1.0.0/ftb-downloader-v1.0.0-x86_64-linux";
            hash = "sha256-kpn2jO+VPL5pYY27oPDSGQndm7bpK7SGfQMDbsivM10=";
          };
        in 
          pkgs.stdenv.mkDerivation {
            pname = "${project_name}";
            version = "${version}";
            src = bin;
            dontUnpack = true;

            nativeBuildInputs = [
              pkgs.copyDesktopItems
              pkgs.autoPatchelfHook
            ] ++ nativeBuildInputs;

            inherit buildInputs;

            desktopItems = [
              desktopEntry
            ];

            installPhase = ''
              runHook preInstall

              mkdir -p $out/bin
              cp $src $out/bin/${project_name}
              chmod +x $out/bin/${project_name}

              runHook postInstall
            '';

            postFixup = ''
              patchelf --set-rpath "${libraryPath}" $out/bin/${project_name}
            '';
          };
      
      apps.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/${project_name}";
      };
    });
}
