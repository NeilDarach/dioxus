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
      dioxus-0-7 = final: prev: {
        inherit (nixpkgs-dioxus.legacyPackages.${prev.stdenv.hostPlatform.system})
          dioxus-cli;
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ dioxus-0-7 dioxus.overlays.default ];
      };
    in {
      devShells.${system}.default = let applyDioxus = dioxus.addToShell pkgs;
      in pkgs.mkShell (applyDioxus { packages = with pkgs; [ just bacon ]; });
    };
}

