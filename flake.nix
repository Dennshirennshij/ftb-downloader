{
  description = "Development environment for raylib on NixOS with wayland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            wget
            curl
            pango
            gtk3
          ];

          nativeBuildInputs = with pkgs; [ pkg-config ];
          LD_LIBARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          env = {
            OPENSSL_LIB = "${pkgs.openssl.dev}/";
          };

          shellHook = ''
          '';
        };
      }
    );
}
