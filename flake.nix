{
  description = "things";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs =
    inputs@{ self
    , nixpkgs
    , crane
    , fenix
    , parts
    , ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      imports = [
      ];
      perSystem = { config, pkgs, system, lib, ... }:
        let
          
          avr-toolchain-plain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-xkgqbv2/b5sQHWHq4eYPI/n3qgL5iyl37BqOEf0Fda8=";
          };
          native-toolchain = (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            # "rust-src"
            # "rustc"
            "rustfmt"
            "llvm-tools-preview"
            "rust-analyzer"
          ]);
          avr-toolchain = pkgs.runCommand "turbowaker-rust" { } ''
            echo "test $out ${avr-toolchain-plain}"
            cp -RL ${avr-toolchain-plain} $out
            chmod -R +rwx $out

            echo "doing patch"

            patch $out/lib/rustlib/src/rust/library/core/Cargo.toml ${./turbowaker/Cargo.toml.patch}
            patch $out/lib/rustlib/src/rust/library/core/src/task/wake.rs ${./turbowaker/wake.rs.patch}
          '';


            toolchain = fenix.packages.${system}.combine [ avr-toolchain native-toolchain ];
            craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

            src = craneLib.cleanCargoSource ./.;

            package = { target ? "thumbv7em-none-eabihf", args ? "", profile ? "release", defmt ? "off" }: craneLib.buildPackage {
              inherit src;

              cargoVendorDir = craneLib.vendorMultipleCargoDeps {
                inherit (craneLib.findCargoFiles src) cargoConfigs;
                cargoLockList = [
                  ./Cargo.lock

                  # Unfortunately this approach requires IFD (import-from-derivation)
                  # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                  # (unless we build with `--impure`).
                  #
                  # Another way around this is to manually copy the rustlib `Cargo.lock`
                  # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                  # will avoid IFD entirely but will require manually keeping the file
                  # up to date!
                  "${toolchain}/lib/rustlib/src/rust/library/Cargo.lock"
                ];
              };

              cargoExtraArgs = "-Z build-std=core,panic_abort,alloc -Z build-std-features=optimize_for_size,panic_immediate_abort,core/turbowakers --target ${target} ${args}";
              CARGO_PROFILE = profile;
              DEFMT_LOG = defmt;
              pname = "rusty-glove";
              version = "0.1.0";

              strictDeps = true;
              doCheck = false;
              buildInputs = [
                # Add additional build inputs here
              ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
                # Additional darwin specific inputs can be set here
                pkgs.libiconv
              ];
            };
            elf = pkg: name: binname: pkgs.runCommandLocal "mkelf" { } ''
              mkdir -p $out
              cp ${pkg}/bin/${name} $out/${binname}.elf
            '';
            binary = pkg: name: pkgs.runCommandLocal "mkbinary" { buildInputs = [ pkgs.llvm ]; } ''
              mkdir -p $out
              llvm-objcopy -O binary ${pkg}/bin/${name} $out/${name}.bin
            '';
          avrlibc = pkgs.pkgsCross.avr.libcCross;
          dfp = pkgs.fetchzip {
            url = "http://packs.download.atmel.com/Atmel.ATtiny_DFP.2.0.368.atpack";
            stripRoot = false;
            extension = "zip";
            sha256 = "sha256-T+HZFUZ8QrCFybIkbUGfoITNgJ5vUcb+t4SlvPstPOE=";
          };
          avr_incflags = [
            "-isystem ${avrlibc}/avr/include"
            "-isystem ${dfp}/include/avr/iotn1616.h"
            "-B${avrlibc}/avr/lib/avr5"
            "-L${avrlibc}/avr/lib/avr5"
            "-B${avrlibc}/avr/lib/avr35"
            "-L${avrlibc}/avr/lib/avr35"
            "-B${avrlibc}/avr/lib/avr51"
            "-L${avrlibc}/avr/lib/avr51"
            "-L${avrlibc}/avr/lib/avrxmega3"
            "-L${dfp}/gcc/dev/attiny1616/avrxmega3/"
          ];
          avrgcc-wrapper = pkgs.stdenv.mkDerivation rec {
            name = "avrgcc-wrapper-${version}";
            version = lib.strings.getVersion pkgs.pkgsCross.avr.buildPackages.gcc-unwrapped;
            nativeBuildInputs = [ pkgs.makeWrapper ];
            buildCommand = ''
            mkdir -p $out/bin
            for exe in gcc g++; do
            makeWrapper "${pkgs.pkgsCross.avr.buildPackages.gcc-unwrapped}/bin/avr-$exe" "$out/bin/avr-$exe" --add-flags "-B${avrlibc}/avr/lib -isystem ${avrlibc}/avr/include -isystem ${dfp}/include/avr -B${dfp}/gcc/dev/attiny1616/avrxmega3 -L${dfp}/gcc/dev/attiny1616/avrxmega3"
            done
            ln -s ${pkgs.pkgsCross.avr.buildPackages.gcc-unwrapped}/lib $out
            '';
          };
        in
        # rec
        {
          devShells.default = craneLib.devShell {
            packages = with pkgs; [
              pkgsCross.avr.libcCross
              pkgsCross.avr.buildPackages.binutils
              #pkgsCross.avr.buildPackages.gcc-unwrapped 
              avrgcc-wrapper
              #avrlibc
              avrdude
              cargo-binutils
            ];
            #CFLAGS = "";
            NIX_CFLAGS_COMPILE = avr_incflags;
            AVR_CFLAGS = avr_incflags;
            AVR_ASFLAGS = avr_incflags;
            AVR_LDFLAGS = avr_incflags;
            ATTINY_DFP = dfp;
          };

          # packages.bin = package {
          #       args = "";
          #     };

          # devShells.default = pkgs.pkgsCross.avr.mkShell {
          #
          #   # inputsFrom = [ (package { profile = "dev"; }) ];
          #   nativeBuildInputs = with pkgs; [
          #     # fenix.packages.${system}.rust-analyzer
          #     cargo-binutils
          #     probe-rs
          #     # ravedude
          #     # pkgsCross.avr.libcCross
          #     # pkgsCross.avr.buildPackages.binutils
          #     # pkgsCross.avr.buildPackages.gcc
          #   ];
          # };
          # packages.default = package { profile = "dev"; };
        };
    };
}
