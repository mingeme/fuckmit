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
      version = "0.2.1";
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
                  "darwin-aarch64" = "0akgs63zbildxpdk21aj7hbkxxrlkawzhdf5mwchrqymhphqp6l6";
                  "darwin-x86_64" = "16gqqlzf6iykypkckvn3dy87dihs1li7i9bgwhaiq9mh79sz41c1";
                  "linux-aarch64" = "02inpj3adbr6qdwf0c33zy57cniqz2bp839fid7h1nw2nd7rq5i2";
                  "linux-x86_64" = "1f724gzm8k6qa69g9ncy40h39pk1lz2cwlaf9hqiqq1qlcmrk8km";
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
