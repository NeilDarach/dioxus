{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust = {
      url = "github:NeilDarach/flakes?dir=rust";
      #url = "path:./rust";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ ] ++ (inputs.rust.overlays.withExtensions {
            ext = [ "rustfmt" "rust-src" "clippy" ];
          });
          config = { };
        };
        devShells.default =
          inputs.rust.makeDevShell pkgs { packages = [ pkgs.dioxus-cli ]; };
      };
    };
}

