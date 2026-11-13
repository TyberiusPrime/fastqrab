{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-26.05";
    utils.url = "github:numtide/flake-utils";
    #naersk.url = "github:nmattia/naersk";
    naersk.url = "github:nix-community/naersk/pull/391/head";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      rust-overlay,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        #pkgs = nixpkgs.legacyPackages."${system}";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable."1.93.1".default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
          extensions = [
            "llvm-tools-preview"
            "rust-analyzer"
          ];
        };

        # Nightly toolchain for the `nightly` devShell.
        rust-nightly = pkgs.rust-bin.nightly.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
          extensions = [
            "llvm-tools-preview"
            "rust-analyzer"
          ];
        };

        # Minimal toolchain for Windows cross-check (no extras needed).
        rust-windows = pkgs.rust-bin.stable."1.93.1".minimal.override {
          targets = [ "x86_64-pc-windows-gnu" ];
        };

        # MinGW cross-compiler from nixpkgs.
        mingw = pkgs.pkgsCross.mingwW64.stdenv.cc;

        naersk-lib-windows = naersk.lib."${system}".override {
          cargo = rust-windows;
          rustc = rust-windows;
        };

        # Override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };

        bacon = pkgs.bacon;

        # cargo-afl is not in nixpkgs, so we build it from the crates.io tarball.
        # The build produces just the `cargo-afl` binary — it does NOT compile
        # aflplusplus (build.rs only does that during `cargo install`, which we
        # bypass). Instead, we populate the xdg data dir cargo-afl looks in
        # with symlinks to `pkgs.aflplusplus` and wrap cargo-afl so it finds
        # them (plus the nix `cargo`, otherwise it panics with NotPresent).
        cargo-afl-unwrapped = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-afl";
          version = "0.18.1";
          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-W2ELM28vHs8xjgh0gRyH/O17kDgMFxKNOnnlbputQb0=";
          };
          cargoLock.lockFile = "${src}/Cargo.lock";
          doCheck = false;
        };

        # cargo-afl-common uses `rustc-<semver>-<short-hash>/afl.rs-<ver>` as
        # the xdg subdirectory. Extract it from the pinned rust toolchain so
        # the path matches at runtime.
        aflRustcDir = pkgs.lib.removeSuffix "\n" (builtins.readFile (pkgs.runCommand "afl-rustc-dir" { } ''
          ${rust}/bin/rustc -vV | ${pkgs.gawk}/bin/awk '
            /^rustc/ { ver=$2 }
            /^commit-hash:/ { printf "rustc-%s-%s", ver, substr($2, 1, 7) }
          ' > $out
        ''));

        aflXdgDataHome = pkgs.runCommand "afl-xdg-data-home" { } ''
          base=$out/afl.rs/${aflRustcDir}/afl.rs-${cargo-afl-unwrapped.version}
          mkdir -p "$base/afl/bin" "$base/afl-llvm"
          for b in ${pkgs.aflplusplus}/bin/afl-*; do
            ln -s "$b" "$base/afl/bin/$(basename "$b")"
          done
          ln -s ${pkgs.aflplusplus}/lib/afl/afl-compiler-rt.o "$base/afl-llvm/afl-compiler-rt.o"
        '';

        cargo-afl = pkgs.runCommand "cargo-afl-wrapped" {
          nativeBuildInputs = [ pkgs.makeWrapper ];
          inherit (cargo-afl-unwrapped) version meta;
          pname = "cargo-afl";
        } ''
          mkdir -p $out/bin
          makeWrapper ${cargo-afl-unwrapped}/bin/cargo-afl $out/bin/cargo-afl \
            --set-default XDG_DATA_HOME ${aflXdgDataHome} \
            --set-default CARGO ${rust}/bin/cargo \
            --prefix PATH : ${pkgs.lib.makeBinPath [ rust pkgs.aflplusplus ]}
        '';

        # Source filtered for the docker binary build: strips directories that
        # are not needed to compile the Rust workspace (dev tooling, test data,
        # documentation, build artefacts).  cookbooks/ must stay because it is
        # embedded via include_str!().
        dockerSrc = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            let
              baseName = baseNameOf (toString path);
              excludedDirs = [ "dev" "test_cases" "docs" "target" "target_claude" ".git" ".jj" ".github" ];
            in
            !(type == "directory" && builtins.elem baseName excludedDirs);
          name = "fastqrab-docker-src";
        };

       cargo-testmap= pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-testmap";
          version = "0.2.0";
          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-4YaqriXA526ZzBOrtrspwpKPhnDmuFT2v5HY+3fF6V4=";
          };
          cargoLock.lockFile = "${src}/Cargo.lock";
          doCheck = false;
        };


        # Shared developer tooling for the stable and nightly devShells.
        # The rust toolchain itself is appended per-shell.
        devTools = with pkgs; [
          bacon
          bash
          aflplusplus
          cargo-afl
          cargo-audit
          cargo-bloat
          cargo-crev
          cargo-deny
          cargo-features-manager
          cargo-flamegraph
          cargo-insta
          cargo-license
          cargo-llvm-cov
          cargo-llvm-lines
          lcov
          cargo-machete
          cargo-mutants
          cargo-nextest
          cargo-outdated
          cargo-shear
          cargo-testmap
          #cargo-udeps
          cargo-vet
          cmake
          gcc
          gnumake
          git
          hugo
          jq
          mold
          openssl
          pkg-config
          samply
          (python315.withPackages (
            ps: with ps; [
              #scipy
              #anndata
              pysam
              #pandas
              toml
            ]
          ))
          #rapidgzip
          which
          ripgrep
          #rust.rust-analyzer
          shellcheck
        ];
      in
      rec {
        # `nix build`
        packages.fastqrab = naersk-lib.buildPackage {
          pname = "fastqrab";
          root = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            gcc
            gnumake
          ];
          buildInputs = with pkgs; [
            openssl
            rapidgzip
            which
            mold
          ];
          release = true;
          CARGO_PROFILE_RELEASE_debug = "0";
          COMMIT_HASH = self.rev or (pkgs.lib.removeSuffix "-dirty" self.dirtyRev or "unknown-not-in-git");
          postInstall =''
              rm $out/bin/fastqrab_alloc_accounting
            '';

        };
        packages.fastqrab_other_linux =
          (naersk-lib.buildPackage {
            pname = "fastqrab";
            root = dockerSrc;
            nativeBuildInputs = with pkgs; [
              pkg-config
              cmake
              gcc
              gnumake
              mold
            ];
            buildInputs = with pkgs; [ openssl ];
            release = true;
            CARGO_PROFILE_RELEASE_debug = "0";
            COMMIT_HASH = self.rev or (pkgs.lib.removeSuffix "-dirty" self.dirtyRev or "unknown-not-in-git");
          }).overrideAttrs
            {
              # make it compatible with other linuxes. It's statically linked anyway
              postInstall = ''
                patchelf $out/bin/fastqrab --set-interpreter "/lib64/ld-linux-x86-64.so.2"
                patchelf $out/bin/fastqrab-decompressor --set-interpreter "/lib64/ld-linux-x86-64.so.2"
                rm $out/bin/fastqrab_alloc_accounting
              '';
            };
        packages.fastqrab-docker =
          let
            binary = packages.fastqrab_other_linux;
          in
          pkgs.dockerTools.buildLayeredImage {
            name = "fastqrab";
            tag = "latest";
            # provide a minimal base with glibc and a busybox shell
            contents = [
              pkgs.busybox
              pkgs.bash
              pkgs.glibc
              pkgs.python3
              pkgs.rapidgzip
              pkgs.dockerTools.fakeNss
              binary
            ];
            config = {
              Env = [ "PATH=/usr/local/bin:/bin" ];
              Entrypoint = [ "/bin/fastqrab" ];
              WorkingDir = "/work";
              User = "nobody";
            };
          };
        packages.check = naersk-lib.buildPackage {
          src = ./.;
          mode = "check";
          name = "fastqrab";
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            gcc
            gnumake
            zstd.bin
            mold
          ];
          buildInputs = with pkgs; [ openssl ];
        };
        # Cross-compile check for Windows (x86_64-pc-windows-gnu / MinGW).
        # Catches cfg(windows) / type / API issues without needing a real Windows runner.
        # Usage: nix build .#check-windows
        # Cross-compile check for Windows (x86_64-pc-windows-gnu / MinGW).
        # Catches cfg(windows) / type / API issues without needing a real Windows runner.
        # Usage: nix build .#check-windows
        packages.check-windows = naersk-lib-windows.buildPackage {
          src = ./.;
          mode = "check";
          name = "fastqrab-windows-check";
          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER =
            "${mingw}/bin/x86_64-w64-mingw32-gcc";
          nativeBuildInputs = [ mingw pkgs.pkg-config ];
        };

        packages.test = naersk-lib.buildPackage {
          # not using naersk test mode, it eats the binaries, we need that binary
          pname = "fastqrab";
          root = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            gcc
            gnumake
            python3
            rapidgzip
            which
            mold
            pkgs.shellcheck
            hugo
          ];
          buildInputs = with pkgs; [ openssl ];
          release = true;
          CARGO_PROFILE_RELEASE_debug = "0";
          COMMIT_HASH = self.rev or (pkgs.lib.removeSuffix "-dirty" self.dirtyRev or "unknown-not-in-git");
          RUST_LOG = "trace";
          # every other test happens wit hteh rapidgzip in the path.
          postInstall = ''
            # run the friendly panic test, expect a non 0 return code.
            # capture stderr

            result=$( { cargo run --release --bin fastqrab -- --test-friendly-panic; } 2>&1 ) || status=$? : "${"status:=0"}"
            if [ "$status" -eq 0 ]; then
              echo "Unexpected success"
              exit 1
            fi
            if [[ ! $result =~ "To help us diagnose the problem" ]]; then
                echo "Error: friendly panic message ' not found in stderr"
                echo "output was '$result'"
                exit 1
            fi

            # The Hugo site references generated cookbook pages, template.toml
            # and schema.json that are gitignored (regenerated locally by
            # developers) and therefore aren't part of the flake's filtered
            # (git-tracked-only) source. Regenerate them here so
            # test_hugo_builds_documentation_site has something to build.
            python3 dev/ci/generate_local_docs.py

            cargo test --release

            # replays docs/content/docs/development/custom_transformation.md
            # (ignored by default - it builds fastqrab several times)
            cargo test --release --test custom_transformation_guide -- --ignored

          '';

        };

        defaultPackage = packages.fastqrab;

        # `nix run`
        apps.fastqrab = utils.lib.mkApp { drv = packages.fastqrab; };
        defaultApp = apps.fastqrab;

        # Cross-compile + Wine shell for local Windows test simulation.
        # Builds with x86_64-pc-windows-gnu (MinGW) and runs tests under Wine.
        # Usage: nix develop .#windows-test --command \
        #   cargo test --target x86_64-pc-windows-gnu --release 2>&1 | tee /tmp/win-test.txt
        devShells.windows-test =
          let
            pthreads = pkgs.pkgsCross.mingwW64.windows.pthreads;
            # GCC 14 in nixpkgs defaults to MCF threading; libgcc_eh.a references
            # _MCF_* symbols that live here.
            mcfgthreads = pkgs.pkgsCross.mingwW64.windows.mcfgthreads;
          in
          pkgs.mkShell {
            nativeBuildInputs = [
              rust-windows
              mingw
              pkgs.wine64
              pkgs.pkg-config
            ];
            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${mingw}/bin/x86_64-w64-mingw32-gcc";
            # libgcc_eh.a references _MCF_* (MCF threading model in GCC 14).
            # libmcfgthread.a in turn needs libntdll.a (__imp_NtWaitForKeyedEvent etc.).
            # Rust adds -lntdll before our link-args, so GNU ld's single-pass scan
            # exhausts libntdll.a before libmcfgthread.a is even linked.
            # --start-group/--end-group forces repeated passes so the three archives
            # can satisfy each other; the whole group lands after -lgcc_eh via link-arg.
            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L ${pthreads}/lib -L ${mcfgthreads}/lib -C link-arg=-Wl,--start-group,-Bstatic,-lmcfgthread,-Bdynamic,-lntdll,-lkernel32,--end-group";
            shellHook = ''
              export WINEPREFIX="$HOME/.wine-fastqrab-test"
            '';
          };

        # Minimal shell for cargo-deny CI check — avoids pulling in cargo-afl
        # and the rest of the full devShell.  Usage: nix develop .#deny
        devShells.deny = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.cargo-deny
            pkgs.git
            rust
          ];
        };

        # `nix develop`
        devShell = pkgs.mkShell {
          COMMIT_HASH = self.rev or (pkgs.lib.removeSuffix "-dirty" self.dirtyRev or "unknown-not-in-git");
          # we only link with mold in our dev environment for build speed. CI can use the old school rust linker
          shellHook = ''
            export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
            # Set shell for cmake builds
            export CONFIG_SHELL="${pkgs.bash}/bin/bash"
            export SHELL="${pkgs.bash}/bin/bash"
          '';
          # supply the specific rust version
          nativeBuildInputs = devTools ++ [ rust ];
        };

        # `nix develop .#nightly` — same tooling as the default devShell but on
        # the latest nightly rust toolchain.
        devShells.nightly = pkgs.mkShell {
          COMMIT_HASH = self.rev or (pkgs.lib.removeSuffix "-dirty" self.dirtyRev or "unknown-not-in-git");
          # we only link with mold in our dev environment for build speed. CI can use the old school rust linker
          shellHook = ''
            export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
            # Set shell for cmake builds
            export CONFIG_SHELL="${pkgs.bash}/bin/bash"
            export SHELL="${pkgs.bash}/bin/bash"
          '';
          nativeBuildInputs = devTools ++ [ rust-nightly ];
        };
      }
    );
}
# {
