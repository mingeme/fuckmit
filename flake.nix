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

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    let
      version = "0.1.2";
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            libiconv
            git
          ] ++ lib.optionals stdenv.isDarwin [
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
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ openssl ] ++
              lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];
            
            doCheck = false;

            meta = with pkgs.lib; {
              description = "AI-powered git commit message generator";
              homepage = "https://github.com/mingeme/fuckmit";
              license = licenses.mit;
              maintainers = [mingeme];
            };
          };
          
          fuckmit = pkgs.stdenv.mkDerivation {
            pname = "fuckmit";
            version = version;
            
            src = let
              baseUrl = "https://github.com/mingeme/fuckmit/releases/download/v${version}";

              hashes = {
                "darwin-aarch64" = "0vq5s5kyb95yw6l92mv77xgzizgpyj44671xipxfl14vz77zgf3q";
                "darwin-x86_64" = "0kdp0195ypa0k5qkvzxsabq0xpc2ipkvph9ryy7qq32fd57lzix5";
                "linux-aarch64" = "018bn8722pxyf8f7qlaf3g3b0gkpwh1nii87zqm685vrqfvcjr8g";
                "linux-x86_64" = "0hsnv2y7b25zrkc8b4p2jbydmlkphxqji5ypmiz2x4xjr9mvlila";
              };
              
              arch = if pkgs.stdenv.isDarwin then
                if pkgs.stdenv.isAarch64 then "darwin-aarch64"
                else "darwin-x86_64"
              else
                if pkgs.stdenv.isAarch64 then "linux-aarch64"
                else "linux-x86_64";
                
              binaryName = "fuckmit-${arch}-${version}.tar.gz";
              hash = hashes.${arch};
            in pkgs.fetchurl {
              url = "${baseUrl}/${binaryName}";
              sha256 = hash;
            };
            
            sourceRoot = "."; 
            
            nativeBuildInputs = [ pkgs.gnutar pkgs.gzip ];
            
            installPhase = ''
              mkdir -p $out/bin
              tar -xzf $src -C $out/bin
              chmod +x $out/bin/fuckmit
            '';
            
            meta = with pkgs.lib; {
              description = "AI-powered git commit message generator";
              homepage = "https://github.com/mingeme/fuckmit";
              license = licenses.mit;
              maintainers = [mingeme];
            };
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.fuckmit;
        };
      });
}
