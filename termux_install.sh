#!/bin/sh
pkg install rust
pkg install sqlite

cargo install diesel_cli --no-default-features --features sqlite

PATH=/data/data/com.termux/files/home/.cargo/bin:$PATH

CARGO_HOME="$HOME/.cargo"
CARGO_BIN="$CARGO_HOME/bin"
INSTALL_DIR="$CARGO_BIN/accountant"

echo "DATABASE_URL=$INSTALL_DIR/.acc.db" > .env
ENV_VAR="DATABASE_URL=$INSTALL_DIR/.acc.db"

diesel setup

cargo build --release

mkdir -p $INSTALL_DIR
cp target/release/accountant $INSTALL_DIR
cp *.db $INSTALL_DIR
cp .env $INSTALL_DIR

echo "alias acc=\"$ENV_VAR $INSTALL_DIR/accountant\"" >> ~/.bashrc
source ~/.bashrc

echo "Successfully installed 'accountant' cli"
echo "Use alias \"acc\" for using app."
