{
  description = "AI-powered git commit message generator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    let
      version = "0.2.0";
      description = "AI-powered git commit message generator";
      homepage = "https://github.com/mingeme/fuckmit";
      license = nixpkgs.lib.licenses.mit;
      maintainers = [ maintainers.mingeme ];
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            [
              rustToolchain
              pkg-config
              openssl
              libiconv
              git
            ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        packages = {
          default = self.packages.${system}.fuckmit;

          fuckmit-from-source = pkgs.rustPlatform.buildRustPackage {
            pname = "fuckmit";
            version = version;
            src = pkgs.lib.cleanSource ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs =
              with pkgs;
              [ openssl ]
              ++ lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

            doCheck = false;

            meta = {
              description = description;
              homepage = homepage;
              license = license;
              maintainers = maintainers;
            };
          };

          fuckmit = pkgs.stdenv.mkDerivation {
            pname = "fuckmit";
            version = version;

            src =
              let
                baseUrl = "https://github.com/mingeme/fuckmit/releases/download/v${version}";

                hashes = {
                  "darwin-aarch64" = "18ibv7hm41x5jwdgamiil2sg7m55dz4nrplk2r8i2cgjy2pwidbg";
                  "darwin-x86_64" = "0d0mrqpqqp796kgxcd6m8a420wfh3a5b993yb7z6281g9pfbcfz7";
                  "linux-aarch64" = "0swa73lb8dvgln09qr2755ibvz3ymxr854kv4n0nlgzj1syp688s";
                  "linux-x86_64" = "0c85i1kji2csx7dp6sdrlav0bla4bj605lczml62i44hwa810dl1";
                };

                arch =
                  if pkgs.stdenv.isDarwin then
                    if pkgs.stdenv.isAarch64 then "darwin-aarch64" else "darwin-x86_64"
                  else if pkgs.stdenv.isAarch64 then
                    "linux-aarch64"
                  else
                    "linux-x86_64";

                binaryName = "fuckmit-${arch}-${version}.tar.gz";
                hash = hashes.${arch};
              in
              pkgs.fetchurl {
                url = "${baseUrl}/${binaryName}";
                sha256 = hash;
              };

            sourceRoot = ".";

            nativeBuildInputs = [
              pkgs.gnutar
              pkgs.gzip
            ];

            installPhase = ''
              mkdir -p $out/bin
              tar -xzf $src -C $out/bin
              chmod +x $out/bin/fuckmit
            '';

            meta = {
              description = description;
              homepage = homepage;
              license = license;
              maintainers = maintainers;
            };
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.fuckmit;
        };
      }
    );
}
