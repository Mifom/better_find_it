{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
          packages = with pkgs; [
                # bintools-unwrapped
                clang lld
                pkgconfig
                pre-commit
                alsa-lib
                udev
                vulkan-loader
                xorg.libX11
                xorg.libXrandr
                xorg.libXcursor
                xorg.libXi
                vulkan-validation-layers
                libGL
          ];
          shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${ pkgs.lib.makeLibraryPath packages }"
          '';
        };
      }
    );
}
