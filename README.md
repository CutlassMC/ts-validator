
# Toml-Schema Validator (`ts-validator`)

A command-line tool for validating TOML files against [**TO**ML **S**chema](https://toml-schema.org/) **D**efinitions (TOSD).

## Quickstart

### Installation

#### Cargo Install

```bash
cargo install --git https://github.com/CutlassMC/ts-validator
```

#### Nix Install

```bash
nix profile add github:CutlassMC/ts-validator
```

### Usage

`toml-schema` has 2 (relevant*) subcommands: `validate-toml` and `validate-tosd`, which validate
their respective file types.

#### Validate TOML

```
$ toml-schema validate-toml --help
Validate a toml file against a provided schema. Defaults to using `toml-schema.location` as the schema to read

Usage: toml-schema validate-toml [OPTIONS] [toml]

Arguments:
  [toml]  The toml file to validate. Defaults to reading from stdin [default: -]

Options:
  -s, --schema <schema>  The schema to validate against. If not provided, `toml-schema.location` must be present in the target toml
  -h, --help             Print help

```

#### Validate TOSD

**NOTE**: Due to a limitation in the reference implementation, `typeof` references (and likely others)
are only checked if they are used. This means that the best way to validate a TOSD is to use it on an
example file.

```
$ toml-schema validate-tosd --help
Check if a tosd file is valid

Usage: toml-schema validate-tosd [tosd]

Arguments:
  [tosd]  The tosd file to validate. Defaults to reading from stdin [default: -]

Options:
  -h, --help  Print help

```

#### Misc

* A 3rd command `completions` exists to get shell completions for `toml-schema`:
```
$ toml-schema completions --help
Return shell completions

Usage: toml-schema completions [OPTIONS] <SHELL>

Arguments:
  <SHELL>  Which shell to get the completions for [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -f, --file  Return a filepath to the completion instead of writing the contents to stdout

```

### Schema Example

#### Writing a Schema

##### Toml-Schema Version

The following file header is required to be first in schema files:
```toml
[toml-schema]
  version = "1.0.0"
```

##### Custom Types

```toml
[types]
  [types.ip]
    type = "string"
    pattern = "^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$"

  [types.url]
    type = "string"
    pattern = "[-a-zA-Z0-9@:%._\\+~\\#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\\+.~\\#?&\\=]*)"

  [types.localhost]
    type = "string"
    allowedvalues = [ "localhost" ]

  [types.address]
    oneOf = [ "types.url", "types.url", "types.localhost" ]

  [types.serverType]
    type = "table"
    service.type = "string"
    hostname.type = "string"
    address.type = "types.address"
```

##### Defining Valid Toml Values

```toml
[elements]
  [elements.servers]
    type = "collection"
    keypattern = "^service-[a-zA-Z0-9]+$"
    typeof = "types.serverType"
    minlength = 1
```

##### Combined View

`./server-schema.tosd`:
```toml
[toml-schema]
  version = "1.0.0"

[types]
  [types.ip]
    type = "string"
    pattern = "^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$"

  [types.url]
    type = "string"
    pattern = "[-a-zA-Z0-9@:%._\\+~\\#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\\+.~\\#?&\\=]*)"

  [types.localhost]
    type = "string"
    allowedvalues = [ "localhost" ]

  [types.address]
    oneOf = [ "types.url", "types.url", "types.localhost" ]

  [types.serverType]
    type = "table"
    service.type = "string"
    hostname.type = "string"
    address.type = "types.address"

[elements]
  [elements.servers]
    type = "collection"
    keypattern = "^service-[a-zA-Z0-9]+$"
    typeof = "types.serverType"
    minlength = 1
```

#### Schema Usage

```toml
[toml-schema]
  location = "./server-schema.tosd"

[servers.service-velocity]
  address = "minecraft.site.tld"
  # OR
  address = "0.0.0.0"
```

## Nix Flake Input

```nix
{
  inputs =
    {
      nixpkgs.url = "github:NixOS/nixpkgs/26.05";

      flake-parts.url = "github:hercules-ci/flake-parts";
      flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

      ts-validator.url = "github:CutlassMC/ts-validator";
      ts-validator.inputs.nixpkgs.follows = "nixpkgs";
    };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake
      { inherit inputs; }
      (
        { lib, ... }:
        {
          systems = lib.systems.flakeExposed;

          perSystem =
            { system, inputs', pkgs, ... }:
            {
              devShells.default =
                pkgs.mkShell
                  {
                    packages =
                      with pkgs;
                      [
                        inputs'.ts-validator.packages.default
                        # OR
                        inputs.ts-validator.packages.${system}.default
                      ];
                  };
            };
        }
      );
}
```

