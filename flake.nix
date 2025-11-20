{
  description =
    "Provide a devshell suitable for devenv which allows compiling and running dioxus apps in the iPhone simulator provided by the system XCode";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-unstable = { url = "github:NixOS/nixpkgs/nixos-unstable"; };
    dioxus = {
      url = "github:NeilDarach/flakes?dir=dioxus";
      #url = "path:./dioxus";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { nixpkgs, nixpkgs-unstable, dioxus, ... }:
    let
      #This is only valid for Mac installations
      system = "aarch64-darwin";
      dioxus-0-7-overlay = final: prev:
        let
          upstream =
            nixpkgs-unstable.legacyPackages.${prev.stdenv.hostPlatform.system}.dioxus-cli;
        in {
          dioxus-cli = upstream.overrideAttrs (oldAttrs: {
            patches = oldAttrs.patches ++ [ ./patches/extra_files.patch ];
          });
        };
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ dioxus-0-7-overlay dioxus.overlays.default ];
      };
    in {
      devShells.${system}.default = let applyDioxus = dioxus.addToShell pkgs;
      in pkgs.mkShell (applyDioxus { packages = with pkgs; [ just bacon ]; });
    };
}

