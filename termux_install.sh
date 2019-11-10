#!/bin/bash
## Script to install accountant in termux android.

if [ "$1" = "full" ]; then
pkg install rust
pkg install sqlite

cargo install diesel_cli --no-default-features --features sqlite
fi

CARGO_HOME="$HOME/.cargo"
CARGO_BIN="$CARGO_HOME/bin"
INSTALL_DIR="$CARGO_BIN/accountant"

rm -rf $INSTALL_DIR
mkdir -p $INSTALL_DIR

echo "DATABASE_URL=$INSTALL_DIR/.acc.db" > .env
ENV_VAR="DATABASE_URL=$INSTALL_DIR/.acc.db"

export PATH=/data/data/com.termux/files/home/.cargo/bin:$PATH

diesel setup

cargo build --release

cp target/release/accountant $INSTALL_DIR
cp .env $INSTALL_DIR


if ! (cat ~/.bashrc | grep -q ".cargo/bin:"); then
    echo "export PATH=/data/data/com.termux/files/home/.cargo/bin:\$PATH" >> ~/.bashrc
fi

echo "#!/bin/sh" > $HOME/../usr/bin/acc
echo "#!/bin/sh" > $HOME/../usr/bin/accountant
echo "$ENV_VAR $INSTALL_DIR/accountant \"\$@\"" >> $HOME/../usr/bin/acc
echo "$ENV_VAR $INSTALL_DIR/accountant \"\$@\"" >> $HOME/../usr/bin/accountant
chmod a+x $HOME/../usr/bin/acc
chmod a+x $HOME/../usr/bin/accountant

source ~/.bashrc

echo "Successfully installed 'accountant' cli"
echo "Use 'acc' command to use the application."

if [ "$1" = "rmdeps" ] || [ "$2" = "rmdeps" ]; then
    cargo clean
    SCRIPT=$(readlink -f "$0")
    SCRIPTPATH=$(dirname "$SCRIPT")
    cd $SCRIPTPATH/..
    rm -rf $SCRIPTPATH
    echo "Removed build dependencies"
fi
