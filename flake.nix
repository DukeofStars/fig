{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      naersk,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = (pkgs.rust-bin.stable.latest.default);
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in
      rec {
        packages.fig = naersk-lib.buildPackage {
          src = ./.;
          pname = "fig";
          buildInputs = with pkgs; [ pkg-config openssl ];
        };
        defaultPackage = packages.fig;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" ];
              targets = [ ];
            })
            pkg-config
            openssl
          ];
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
