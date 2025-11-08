{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = inputs@{ nixpkgs, rust-overlay, ... }:
    let
      pkgs = import inputs.nixpkgs {
        #This is only valid for Mac installations
        system = "aarch64-darwin";
        overlays = [
          (import rust-overlay)
          (prev: final: {
            rustToolChain = (prev.rust-bin.stable.latest.default.override {
              extensions = [ "rustfmt" "rust-src" ];
              targets = [
                "wasm32-unknown-unknown"
                "aarch64-apple-ios"
                "aarch64-apple-ios-sim"
              ];
            }).overrideAttrs (oldAttrs: {
              ## Modify the rust package to not propagate the wrapped clang packages
              ## binutils-unwrapped provides install_name_tool which installation needs
              propagatedBuildInputs = [ pkgs.darwin.binutils-unwrapped ];
              depsHostHostPropagated = [ ];
              depsTargetTargetPropagated = [ ];
            });
          })
        ];
      };

      #The default cc is the wrapped clang from nix.
      #Without this link, rustc finds the wrapped cc and
      #that adds macos specific flags which conflict when
      #cross-compiling for iphone
      cclink = pkgs.stdenv.mkDerivation {
        name = "cclink-impure";
        # Fails in sandbox. Use `--option sandbox relaxed` or `--option sandbox false`.
        __noChroot = true;
        buildCommand = ''
          mkdir -p $out/bin
          cd $out/bin
          ln -s "${xcode}/bin/clang" cc
        '';
      };

      # Create symlinks to the system XCode installation
      xcode = (import (nixpkgs + "/pkgs/development/mobile/xcodeenv") {
        inherit (pkgs) callPackage;
      }).composeXcodeWrapper { };
    in {
      devShells.aarch64-darwin.default = pkgs.mkShell {
        packages = with pkgs; [
          xcode
          just
          bacon
          dioxus-cli
          rustToolChain
          cclink
        ];
        shellHook = ''
          RUST_SRC_PATH="${pkgs.rustToolChain}/lib/rustlib/src/rust/library";
          #Use the system installation of the SDKs, not the nix-installed version
          unset DEVELOPER_DIR
          unset SDKROOT
        '';
      };
    };
}

