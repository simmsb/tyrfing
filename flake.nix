{
  description = "things";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
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
          # avr-toolchain = fenix.packages.${system}.fromToolchainFile {
          #   file = ./rust-toolchain.toml;
          #   sha256 = "";
          # };
          # native-toolchain = fenix.packages.${system}.complete.withComponents [
          #   "cargo"
          #   "clippy"
          #   "rust-src"
          #   "rustc"
          #   "rustfmt"
          # ];
          # toolchain = fenix.packages.${system}.combine [ avr-toolchain native-toolchain ];
          # craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          # package = { target ? avr-toolchain.target, args ? "", profile ? "release" }: craneLib.buildPackage {
          #   cargoExtraArgs = "--target ${target} ${args}";
          #   CARGO_PROFILE = profile;
          #   src = craneLib.cleanCargoSource (craneLib.path ./.);
          #   doCheck = false;
          #   buildInputs = [
          #     # Add additional build inputs here
          #   ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          #     # Additional darwin specific inputs can be set here
          #     pkgs.libiconv
          #   ];
          # };
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
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
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
