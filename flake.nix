{
  description = "Forward Progress Modpack Creation Tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Used for rust compiler
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Advisory db from rust-sec
    advisory-db = {
      url = "github:RustSec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-compat, utils, naersk, rust-overlay, advisory-db }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];

        };
        crateName = "ffpack";
        naersk-lib = naersk.lib."${system}".override {
          rustc = pkgs.rust-bin.stable.latest.default;
          cargo = pkgs.rust-bin.stable.latest.default;
        };
        devBase = with pkgs; [
          cargo-audit
          nixpkgs-fmt
          git-chglog
          openssl
          pkgconfig
          pre-commit
          rust-analyzer
          cmake
          cargo-release
          git
          git-lfs
          cargo-udeps
          cbor-diag
          cargo-criterion
          perl
          python39Packages.mdformat
          # for ci reasons
          bash
          cacert
        ];
      in
      rec
      {
        # Main binary
        packages.${crateName} = naersk-lib.buildPackage {
          pname = "${crateName}";
          root = ./.;
          # Enable binary features so we actually get the binary
          cargoBuildOptions = x: x ++ [ ''--features="binary"'' ];
        };
        # binary + tests
        packages.tests.${crateName} = naersk-lib.buildPackage {
          pname = "${crateName}";
          root = ./.;
          doCheck = true;
          # Enable binary features so we actually get the binary
          cargoBuildOptions = x: x ++ [ ''--features="binary"'' ];
        };

        packages.docs.${crateName} = naersk-lib.buildPackage {
          pname = "${crateName}";
          root = ./.;
          dontBuild = true;
          doDoc = true;
          doDocFail = true;
        };

        defaultPackage = packages.${crateName};

        # Make some things eaiser to do in CI
        packages.lints = {
          # lint formatting
          format.${crateName} =
            with import nixpkgs { inherit system; };
            stdenv.mkDerivation {
              name = "format lint";
              src = self;
              nativeBuildInputs = with pkgs; [ rust-bin.stable.latest.default ];
              buildPhase = "cargo fmt -- --check";
              installPhase = "mkdir -p $out; echo 'done'";
            };
          # audit against stored advisory db
          audit.${crateName} =
            with import nixpkgs { inherit system; };
            stdenv.mkDerivation {
              name = "format lint";
              src = self;
              nativeBuildInputs = with pkgs; [ rust-bin.stable.latest.default cargo-audit ];
              buildPhase = ''
                export HOME=$TMP
                mkdir -p ~/.cargo
                cp -r ${advisory-db} ~/.cargo/advisory-db
                cargo audit -n
              '';
              installPhase = "mkdir -p $out; echo 'done'";
            };
          # Clippy
          clippy.${crateName} = naersk-lib.buildPackage {
            pname = "${crateName}";
            root = ./.;
            cargoTestCommands = (old: [ ''cargo $cargo_options clippy'' ]);
            doCheck = true;
            dontBuild = true;
          };
        };


        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          buildInputs =
            [ pkgs.rust-bin.stable.latest.default ] ++ devBase;
        };

        packages.nightlyRustShell = pkgs.mkShell {
          buildInputs =
            [
              (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                extensions = [ "rust-src" "clippy" ];
              }))
            ] ++ devBase;
        };
      });
}
