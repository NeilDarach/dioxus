{
  description =
    "Provide a devshell suitable for devenv which allows compiling and running dioxus apps in the iPhone simulator provided by the system XCode";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      #This is only valid for Mac installations
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
          (final: prev:
            let rustToolChain = prev.rust-bin.stable.latest.default;
            in {
              rustToolChain = (rustToolChain.override {
                extensions = [ "rustfmt" "rust-src" ];
                targets = [
                  "wasm32-unknown-unknown"
                  "aarch64-apple-ios"
                  "aarch64-apple-ios-sim"
                ];
              }).overrideAttrs (oldAttrs: {
                # Modify the rust package to not propagate the wrapped clang package
                # binutils-unwrapped provides install_name_tool which installation requires
                propagatedBuildInputs = [ final.darwin.binutils-unwrapped ];
                depsHostHostPropagated = [ ];
                depsTargetTargetPropagated = [ ];
              });
            })
          (final: prev: {
            # Create symlinks to  the system XCode installation
            system-xcode =
              (import (nixpkgs + "/pkgs/development/mobile/xcodeenv") {
                inherit (final) callPackage;
              }).composeXcodeWrapper { };

            # The default cc points to the wrapped clang from nix.
            # Without an explicit link to the system compiler, rustc finds
            # the wrapped compiler and adds macos specific flags which
            # conflict with the ios flags added by dx
            # Linking to the system file cribbed from xcodeenv
            cclink-impure = final.stdenv.mkDerivation {
              name = "cclink-impure";
              __noChroot = true;
              buildCommand = ''
                mkdir -p $out/bin
                cd $out/bin
                ln -s "${final.system-xcode}/bin/clang" cc
              '';
            };
          })
        ];
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          system-xcode
          cclink-impure
          rustToolChain
          rust-analyzer
          dioxus-cli
        ];
        shellHook = ''
          export RUST_SRC_PATH="${pkgs.rustToolChain}/lib/rustlib/src/rust/library"
          # Use the system installation of the SDKs, not the nix-installed versions
          unset DEVELOPER_DIR
          unset SDKROOT
        '';
      };
    };
}

