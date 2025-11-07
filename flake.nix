{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    darwin = {
      url = "github:nix-darwin/nix-darwin/nix-darwin-25.05";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = inputs@{ nixpkgs, rust-overlay, darwin, ... }:
    let
      pkgs = import inputs.nixpkgs {
        system = "aarch64-darwin";
        #config.replaceStdenv = ({ pkgs }: pkgs.newstdenv);
        overlays = [
          (import rust-overlay)
          (prev: final: {
            rustToolChain = prev.rust-bin.stable.latest.default.override {
              extensions = [ "rustfmt" "rust-src" ];
              targets = [
                "wasm32-unknown-unknown"
                "aarch64-apple-ios"
                "aarch64-apple-ios-sim"
              ];
            };
          })
          (self: super:
            let
              cc = super.stdenv.cc.override {
                extraBuildCommands = ''
                  sed -i '/-mmacos-version-min/d' "''${out}/nix-support/add-flags.sh"
                  echo "#Modified" >> "''${out}/nix-support/add-flags.sh"'';
              };
              newstdenv = super.stdenv.override { inherit cc; };
            in { inherit newstdenv; })
        ];
      };
    in {
      devShells.aarch64-darwin.default = let
        xcodeenv = import (nixpkgs + "/pkgs/development/mobile/xcodeenv") {
          inherit (pkgs) callPackage;
        };
        bare-rust = pkgs.rustToolChain.overrideAttrs (oldAttrs: {
          ## don't propagate any build inputs. this allows us to set gcc in stdenv below
          propagatedBuildInputs = [ pkgs.darwin.binutils-unwrapped ];
          depsHostHostPropagated = [ ];
          depsTargetTargetPropagated = [ ];
        });
      in (pkgs.mkShellNoCC.override { stdenv = pkgs.llvmPackages.stdenv; }) {
        strictDeps = true;

        buildInputs = [ ];
        nativeBuildInputs = [ ];
        packages = [
          (xcodeenv.composeXcodeWrapper { })
          pkgs.just
          pkgs.bacon
          pkgs.dioxus-cli
          bare-rust
          pkgs.llvmPackages.clang-unwrapped
        ];
        shellHook = ''
          RUST_SRC_PATH="${pkgs.rustToolChain}/lib/rustlib/src/rust/library";
          PATH="/Users/neil:''${PATH}"
          unset DEVELOPER_DIR
          unset SDKROOT
          #unset MACOSX_DEPLOYMENT_TARGET
          #NIX_CFLAGS_COMPILE="''${NIX_CFLAGS_COMPILE} -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator26.0.sdk -miphonesimulator-version-min=18.0"
          #NIX_CFLAGS_COMPILE_FOR_BUILD="''${NIX_CFLAGS_COMPILE_FOR_BUILD} -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator26.0.sdk -miphonesimulator-version-min=18.0"

        '';
      };
    };
}

