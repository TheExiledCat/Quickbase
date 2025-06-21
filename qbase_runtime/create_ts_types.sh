SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
SRC_DIR=$SCRIPT_DIR/bindings

TARGET_DIR=$SCRIPT_DIR/webui/src/classes
mkdir -p $TARGET_DIR
cargo test export_bindings
mv $SRC_DIR/* $TARGET_DIR
