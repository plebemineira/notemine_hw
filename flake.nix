{
  description = "A flake for developing notemine_hw on mutinynet signet (30s blocks)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flakebox,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        flakeboxLib = flakebox.lib.${system} { };
        pkgs = import nixpkgs { inherit system; };

        mutinynetBuild = pkgs.stdenv.mkDerivation rec {
          pname = "bitcoind-mutinynet";
          version = "24.99.0";

          src = pkgs.fetchFromGitHub {
            owner = "benthecarman";
            repo = "bitcoin";
            rev = "custom-signet-blocktime";
            sha256 = "Y3PjlKcH5DpfT+d2YAwPylNDJExB8Z0C0E4SB/Lt3vY=";
          };

          nativeBuildInputs = [
            pkgs.autoreconfHook
            pkgs.pkg-config
            pkgs.intltool
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.util-linux ];

          buildInputs = [
            pkgs.boost
            pkgs.db48
            pkgs.sqlite
            pkgs.libevent
            pkgs.miniupnpc
            pkgs.zeromq
            pkgs.zlib
          ];

          configureFlags = [
            "--enable-wallet"
            "--disable-fuzz"
            "--disable-fuzz-binary"
            "--disable-tests"
            "--disable-gui-tests"
            "--disable-bench"
          ];

          checkInputs = [ pkgs.python3 ];

          checkFlags = [ "LC_ALL=en_US.UTF-8" ];

          enableParallelBuilding = true;

          makeFlags = [ "-j8" ];

          meta = {
            description = "A custom Bitcoin Signet with ~30s block time. Maintained by Mutiny";
            license = pkgs.lib.licenses.mit;
            platforms = pkgs.lib.platforms.unix;
          };
        };

        notemine_hw_dir = "/tmp/notemine_hw";
        bitcoin_dir = notemine_hw_dir + "/bitcoin";
        lightning_dir = notemine_hw_dir + "/lighting";
      in
      {
        devShells = flakeboxLib.mkShells {
          buildInputs = [
            pkgs.clightning
            pkgs.lsof
            pkgs.tor
            mutinynetBuild
          ];

          shellHook = ''

            mkdir -p ${notemine_hw_dir}
            mkdir -p ${bitcoin_dir}/signet
            mkdir -p ${lightning_dir}

            # Check if TOR is running, otherwise start it
            if lsof -Pi :9050 -sTCP:LISTEN -t >/dev/null ; then
                echo "TOR proxy is already running."
            else
                echo "Starting TOR proxy..."
                tor --RunAsDaemon 1

                # wait for TOR to bootstrap
                sleep 10
            fi

            # todo accelerate IBD

            if lsof -Pi :38333 -sTCP:LISTEN -t >/dev/null ; then
                echo "Port 38333 is already in use. Not starting bitcoind."
            else
                echo "Port 38333 is free. Starting bitcoind..."
                bitcoind \
                   -signet \
                   -signetchallenge=512102f7561d208dd9ae99bf497273e16f389bdbd6c4742ddb8e6b216e64fa2928ad8f51ae \
                   -addnode=45.79.52.207:38333 \
                   -dnsseed=0 \
                   -signetblocktime=30 \
                   -datadir=${bitcoin_dir} \
                   -daemon

                   # wait bitcoind bootstrap
                   sleep 5
            fi

            alias btc="bitcoin-cli -signet -datadir=${bitcoin_dir}"
            alias cln="lightning-cli --signet --lightning-dir=${lightning_dir}"

            echo "btc getblockchaininfo"
            btc getblockchaininfo

#            LAUNCH_LN="1";

            if [ ! -z "$LAUNCH_LN" ]; then

                if lsof -Pi :39735 -sTCP:LISTEN -t >/dev/null ; then
                    echo "Port 39735 is already in use. Not starting lightningd."
                else
                    echo "Port 39735 is free. Starting lightningd..."
                    lightningd \
                        --signet \
                        --bitcoin-datadir=${bitcoin_dir} \
                        --lightning-dir=${lightning_dir} \
                        --autolisten=true \
                        --log-level=debug \
                        --log-file=${lightning_dir}/debug.log \
                        --daemon
                fi

                echo "cln getinfo"
                cln getinfo
            fi
          '';
        };
      }
    );
}
