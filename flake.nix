{
  description = "A simple flake with scaffolding for standard uses.";

  inputs =
    {
      nixpkgs.url = "github:NixOS/nixpkgs/26.05";

      flake-parts.url = "github:hercules-ci/flake-parts";
      flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

      toml-schema-source.url = "github:brunoborges/toml-schema/v1.0.0-rc.2";
      toml-schema-source.flake = false;

      rust-module.url = "github:MaxTheMooshroom/flake-module-rust";
      rust-module.inputs.flake-parts.follows = "flake-parts";
    };

  outputs =
    { flake-parts, rust-module, toml-schema-source, ... }@inputs:
    flake-parts.lib.mkFlake
      { inherit inputs; }
      (
        { lib, ... }:
        {
          systems = lib.systems.flakeExposed;

          imports = [ rust-module.flakeModule ];

          rust.toolchain.file = ./rust-toolchain.toml;

          perSystem =
            { self', pkgs, rustPlatform, ... }:
            {
              packages =
                {
                  default = self'.packages.ts-validator;

                  ts-validator =
                    rustPlatform.buildRustPackage
                      (
                        finalAttrs:
                        {
                          pname = "ts-validator";
                          version = "0.1.0";
                          src = ./.;

                          nativeBuildInputs =
                            [
                              pkgs.makeWrapper
                              pkgs.installShellFiles
                            ];

                          cargoDeps =
                            pkgs.rustPlatform.importCargoLock
                              {
                                lockFile = ./Cargo.lock;
                                outputHashes."toml-schema-${self'.packages.toml-schema.version}" =
                                  "sha256-G/L0iZp+aRm+BpZlk7CiRc8ipDV9vboM+P+hctNX6zI=";
                              };

                          postInstall = /* bash */ ''
                            completions=$out/share/ts-validator/completions
                            mkdir -p $completions

                            cp target/release-tmp/build/ts-validator-*/out/* $completions/

                            installShellCompletion \
                              --bash target/release-tmp/build/ts-validator-*/out/ts-validator.bash \
                              --zsh target/release-tmp/build/ts-validator-*/out/ts-validator.zsh \
                              --fish target/release-tmp/build/ts-validator-*/out/ts-validator.fish
                          '';

                          postFixup = /* bash */ ''
                            wrapProgram "$out/bin/ts-validator" \
                              --set-default COMPLETIONS_DIR "$out/share/ts-validator/completions"
                          '';
                        }
                      );

                  toml-schema =
                    pkgs.rustPlatform.buildRustPackage
                      (
                        finalAttrs:
                        {
                          pname = "toml-schema";
                          version = "1.0.0-rc.2";

                          src = toml-schema-source;

                          cargoRoot = "reference-implementations/rust";
                          buildAndTestSubdir = finalAttrs.cargoRoot;
                          cargoDeps =
                            pkgs.rustPlatform.fetchCargoVendor
                              {
                                pname = "${finalAttrs.pname}-deps";

                                inherit (finalAttrs)
                                  version
                                  src
                                  cargoRoot
                                  ;

                                hash = "sha256-2+O1y+iSVJMyvLzVpPMvYBl3+HRAbDP0Bbv2wglnt7g=";
                              };
                        }
                      );
                };

              devShells =
                {
                  default =
                    pkgs.mkShell
                      {
                        packages =
                          with pkgs;
                          [
                            self'.rust-bins

                            # Cargo subcommand to show result of
                            # macro expansion
                            cargo-expand

                            # reduces the noise of compiler messages
                            cargo-limit

                            # Generate README.md from docstrings
                            cargo-readme

                            # everything about releasing a rust crate ; like
                            # `cargo publish`, but more comprehensive
                            cargo-release

                            # Checks rust documentation for spelling and
                            # grammar mistakes
                            cargo-spellcheck
                          ];
                      };

                  use =
                    pkgs.mkShell { packages = [ self'.packages.ts-validator ]; };
                };
            };
        }
      );
}
