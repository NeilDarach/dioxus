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
      dioxus-0-7 = final: prev:
        let
          addRustPatches = pkg: patches: cargoHash:
            pkg.overrideAttrs (oldAttrs: rec {
              # take the original source and apply all patches before making it the new source
              # we cannot use patches or patchPhase because all dependencies are vendored into
              # a separate derivation before the patch phase resulting in mismatching Cargo.lock
              # checksums
              # (Not sure about this, original version specified a new hash, but
              # simple code updates don't seem to need it.)
              src = prev.runCommand "patched-source" { } ''
                cp -v --no-preserve=mode -r ${oldAttrs.src} $out
                cd $out
                ${prev.lib.concatMapStringsSep "\n"
                (patch: ''patch -p1 < "${patch}"'') patches}
              '';
              cargoDeps = oldAttrs.cargoDeps.overrideAttrs
                (prev.lib.const { inherit src; });
            });
          cli =
            nixpkgs-unstable.legacyPackages.${prev.stdenv.hostPlatform.system}.dioxus-cli;
        in {
          dioxus-cli = addRustPatches cli [ ./patches/extra_files.patch ] "";
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
        overlays = [ dioxus-0-7 dioxus.overlays.default ];
      };
    in {
      devShells.${system}.default = let applyDioxus = dioxus.addToShell pkgs;
      in pkgs.mkShell (applyDioxus { packages = with pkgs; [ just bacon ]; });
    };
}

