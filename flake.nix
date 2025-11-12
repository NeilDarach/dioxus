{
  description =
    "Provide a devshell suitable for devenv which allows compiling and running dioxus apps in the iPhone simulator provided by the system XCode";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-dioxus.url = "github:CathalMullan/nixpkgs/dioxus-cli-v0.7.0";
    dioxus = {
      url = "github:NeilDarach/flakes?dir=dioxus";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

  };

  outputs = { nixpkgs, nixpkgs-dioxus, dioxus, ... }:
    let
      #This is only valid for Mac installations
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = dioxus.overlays ++ [
          (final: prev: {
            inherit (nixpkgs-dioxus.legacyPackages.${prev.stdenv.hostPlatform.system})
              dioxus-cli;
          })
        ];
      };

    in {
      devShells.${system}.default = pkgs.mkShell (dioxus.addToShell pkgs {
        packages = [ pkgs.rust-analyzer pkgs.just pkgs.bacon ];
      });
    };
}

