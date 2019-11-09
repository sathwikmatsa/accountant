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


if ! (cat ~/.bashrc | grep -q "cargo/bin"); then
    echo "export PATH=/data/data/com.termux/files/home/.cargo/bin:\$PATH" >> ~/.bashrc
fi

echo "#!/bin/sh" > $HOME/../usr/bin/acc
echo "#!/bin/sh" > $HOME/../usr/bin/accountant
echo "$ENV_VAR $INSTALL_DIR/accountant" >> $HOME/../usr/bin/acc
echo "$ENV_VAR $INSTALL_DIR/accountant" >> $HOME/../usr/bin/accountant
chmod a+x $HOME/../usr/bin/acc
chmod a+x $HOME/../usr/bin/accountant

source ~/.bashrc

echo "Successfully installed 'accountant' cli"
echo "Use 'acc' command to use the application."

if [ "$1" = "rmdeps" ]; then
    cargo clean
    SOURCE_DIR=$PWD
    cd ..
    rm -rf $SOURCE_DIR
fi
