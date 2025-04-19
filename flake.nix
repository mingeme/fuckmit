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
          
          # 从源码构建的版本
          fuckmit-from-source = pkgs.rustPlatform.buildRustPackage {
            pname = "fuckmit";
            version = "0.1.1";
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
          
          # 从GitHub下载预编译二进制文件
          fuckmit = pkgs.stdenv.mkDerivation {
            pname = "fuckmit";
            version = "0.1.1";
            
            # 根据系统架构选择不同的二进制文件
            src = let
              baseUrl = "https://github.com/mingeme/fuckmit/releases/download/v0.1.1";
              binaryName = if pkgs.stdenv.isDarwin then
                if pkgs.stdenv.isAarch64 then "fuckmit-darwin-aarch64-0.1.1.tar.gz"
                else "fuckmit-darwin-x86_64-0.1.1.tar.gz"
              else
                if pkgs.stdenv.isAarch64 then "fuckmit-linux-aarch64-0.1.1.tar.gz"
                else "fuckmit-linux-x86_64-0.1.1.tar.gz";
            in pkgs.fetchurl {
              url = "${baseUrl}/${binaryName}";
              # 这里需要替换为实际的SHA256哈希值
              sha256 = "0000000000000000000000000000000000000000000000000000";
            };
            
            # 只需要基本的解压工具
            nativeBuildInputs = [ pkgs.gnutar pkgs.gzip ];
            
            # 简单的安装步骤
            installPhase = ''
              mkdir -p $out/bin
              tar -xzf $src
              install -Dm755 fuckmit $out/bin/fuckmit
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
