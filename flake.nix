{
  description = "Build a cargo workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        inherit (pkgs) lib;

        # From https://github.com/ipetkov/crane/blob/fa8b7445ddadc37850ed222718ca86622be01967/docs/advanced/overriding-function-behavior.md?plain=1#L20C1-L28C6
        craneLib = (crane.mkLib pkgs).overrideScope (final: prev: {
          # TODO figure out a way to build a debug and release profile of things
          mkCargoDerivation = args: prev.mkCargoDerivation ({
            CARGO_PROFILE = "dev";
          } // args);
        });

        jsonFilter = path: _type: builtins.match ".*json$" path != null;

        srcFilter = path: type: (jsonFilter path type) || (craneLib.filterCargoSources path type);

        src = lib.cleanSourceWith {
          src = ./.;
          filter = srcFilter;
          name = "source";
        };

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = [ ] ++ lib.optionals pkgs.stdenv.isLinux [
            pkgs.mold-wrapped
            pkgs.lld
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies (of the entire workspace),
        # so we can reuse all of that work (e.g. via cachix) when running in CI
        # It is *highly* recommended to use something like cargo-hakari to avoid
        # cache misses when building individual top-level-crates
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

        fileSetForCrate = crate: lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            ./hakari
            ./api
            crate
          ];
        };

        # Build the top-level crates of the workspace as individual derivations.
        # This allows consumers to only depend on (and build) only what they need.
        # Though it is possible to build the entire workspace as a single derivation,
        # so this is left up to you on how to organize things
        open-webui-cli = craneLib.buildPackage (individualCrateArgs // {
          pname = "open-webui-cli";
          cargoExtraArgs = "-p open-webui-cli";
          src = fileSetForCrate ./cli;
        });

        open-webui-cli-release = craneLib.buildPackage (individualCrateArgs // {
          pname = "open-webui-cli";
          cargoExtraArgs = "-p open-webui-cli";
          src = fileSetForCrate ./cli;
          CARGO_PROFILE = "release";
        });

        staticEnv = {
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
        };

        # TODO need to make this work across x86_64 and arm on linux
        nativeToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
        };

        craneLibStatic = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
        });

        open-webui-cli-static = craneLibStatic.buildPackage (commonArgs // staticEnv // {
          pname = "open-webui-cli-static";
          cargoExtraArgs = "-p open-webui-cli";
          src = fileSetForCrate ./cli;
          CARGO_PROFILE = "release";
        });

        # Version of open-webui I snagged the api from
        openwebuiver = "0.3.35";
      in
      {
        checks = {
          # Build the crates as part of `nix flake check` for convenience
          inherit open-webui-cli;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          open-webui-cli-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          open-webui-cli-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          open-webui-cli-fmt = craneLib.cargoFmt {
            inherit src;
          };

          open-webui-cli-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          open-webui-cli-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          open-webui-cli-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on other crate derivations
          # if you do not want the tests to run twice
          open-webui-cli-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });

          # Ensure that cargo-hakari is up to date
          open-webui-cli-hakari = craneLib.mkCargoDerivation {
            inherit src;
            pname = "open-webui-cli-hakari";
            cargoArtifacts = null;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
              cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
              cargo hakari verify
            '';

            nativeBuildInputs = [
              pkgs.cargo-hakari
            ];
          };
        };

        packages = {
          inherit open-webui-cli;
          default = open-webui-cli;
          release = open-webui-cli-release;
        } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          open-webui-cli-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        } // lib.optionalAttrs (pkgs.stdenv.isLinux) {
          static = open-webui-cli-static;
        };

        apps = {
          open-webui-cli = flake-utils.lib.mkApp {
            drv = open-webui-cli;
          };
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = with pkgs; [
            cargo-hakari
            openapi-generator-cli
            (pkgs.writeScriptBin "fmtall" ''
              taplo fmt
              cargo fmt
            '')
            (pkgs.writeScriptBin "rebuild-api-crates" ''
              set -xe
              printf "removing existing generated code\n"
              rm -fr api
              ${pkgs.openapi-generator-cli}/bin/openapi-generator-cli generate --input-spec openapi/${openwebuiver}/ollama/openapi.json --generator-name rust --output api/ollama --package-name ollama
              ${pkgs.openapi-generator-cli}/bin/openapi-generator-cli generate --input-spec openapi/${openwebuiver}/rag/openapi.json --generator-name rust --output api/rag --package-name rag

              # This spec is invalid? wtf
              ${pkgs.openapi-generator-cli}/bin/openapi-generator-cli generate --input-spec openapi/${openwebuiver}/webui/openapi.json --generator-name rust --output api/webui --package-name webui --skip-validate-spec

              ${pkgs.openapi-generator-cli}/bin/openapi-generator-cli generate --input-spec openapi/${openwebuiver}/default/openapi.json --generator-name rust --output api/default --package-name default

              # Add reqwest feature stream so we can fix the file params that take a PathBuf object
              # The openapi-generator-cli (actually the java backing it) needs a patch.
              for api in api/*; do
                (cd $api &&
                   cargo add reqwest --no-default-features --features rustls-tls,stream,json
                )
              done

              # I dunno wat the real license should be its generated who cares.
              # TODO set this to whatever open-webui is to be "safe"
              sed -i -e "s/Unlicense/BlueOak-1.0.0/g" api/*/Cargo.toml

              taplo fmt
              cargo fmt

              # cargo fix at some point?

              # Patch the incompetently produced openapi code from
              # openapi-generate-cli to actually work, TODO upstream the fixes
              # to the openapi-generator to be able to use paths properly.
              for patchit in default ollama webui; do
                patch -p1 < ./patches/${openwebuiver}/$patchit.patch
              done

              sed -i '3s/^/#![allow(non_snake_case)]\n/' api/*/src/lib.rs

              # If theres uncommitted files watever
              #cargo fix --allow-dirty
            '')
          ];
        };
      });
}
