#!/bin/bash
pkg install rust
pkg install sqlite

cargo install diesel_cli --no-default-features --features sqlite

CARGO_HOME="$HOME/.cargo"
CARGO_BIN="$CARGO_HOME/bin"
INSTALL_DIR="$CARGO_BIN/accountant"

mkdir -p $INSTALL_DIR

echo "DATABASE_URL=$INSTALL_DIR/.acc.db" > .env
ENV_VAR="DATABASE_URL=$INSTALL_DIR/.acc.db"

export PATH=/data/data/com.termux/files/home/.cargo/bin:$PATH

diesel setup

cargo build --release

cp target/release/accountant $INSTALL_DIR
cp .env $INSTALL_DIR

cargo clean

if ! (cat ~/.bashrc | grep -q "alias acc"); then
    echo "alias acc=\"$ENV_VAR $INSTALL_DIR/accountant\"" >> ~/.bashrc
fi
if ! (cat ~/.bashrc | grep -q "cargo/bin"); then
    echo "export PATH=/data/data/com.termux/files/home/.cargo/bin:\$PATH" >> ~/.bashrc
fi

source ~/.bashrc

echo "Successfully installed 'accountant' cli"
echo "Use alias \"acc\" for using app."
