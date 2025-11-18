{
  description =
    "Provide a devshell suitable for devenv which allows compiling and running dioxus apps in the iPhone simulator provided by the system XCode";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-dioxus = { url = "github:CathalMullan/nixpkgs/dioxus-cli-v0.7.0"; };
    dioxus = {
      url = "github:NeilDarach/flakes?dir=dioxus";
      #url = "path:./dioxus";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { nixpkgs, nixpkgs-dioxus, dioxus, ... }:
    let
      #This is only valid for Mac installations
      system = "aarch64-darwin";
      dioxus-0-7 = final: prev:
        let
          cli =
            nixpkgs-dioxus.legacyPackages.${prev.stdenv.hostPlatform.system}.dioxus-cli;
        in {
          dioxus-cli = cli.overrideAttrs (oldAttrs: {
            cargoPatches = (oldAttrs.cargoPatches or [ ])
              ++ [ ./patches/extra_files.patch ];
          });
        };
      dioxus-local = final: prev: {
        dioxus-cli = final.stdenv.mkDerivation {
          name = "dioxus-cli";
          __noChroot = true;
          buildCommand = ''
            mkdir -p $out/bin
            cd $out/bin
            ln -s "/Users/neil/projects/dx/target/debug/dx" dx
          '';
        };
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ dioxus-local dioxus.overlays.default ];
      };
    in {
      devShells.${system}.default = let applyDioxus = dioxus.addToShell pkgs;
      in pkgs.mkShell (applyDioxus { packages = with pkgs; [ just bacon ]; });
    };
}

