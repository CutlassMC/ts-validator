{
  description = "A simple flake with scaffolding for standard uses.";

  inputs =
    {
      nixpkgs.url = "github:NixOS/nixpkgs/26.05";

      flake-parts.url = "github:hercules-ci/flake-parts";
      flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

      rust-module.url = "github:MaxTheMooshroom/flake-module-rust";
      rust-module.inputs.flake-parts.follows = "flake-parts";
    };

  outputs =
    { flake-parts, rust-module, ... }@inputs:
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
                                outputHashes."toml-schema-1.0.0-rc.2" =
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
                            wrapProgram "$out/bin/toml-schema" \
                              --set-default COMPLETIONS_DIR "$out/share/ts-validator/completions"
                          '';
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

                            cargo-workspaces

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
