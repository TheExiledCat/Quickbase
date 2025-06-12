SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
SRC_DIR=$SCRIPT_DIR/bindings
TARGET_DIR=$SCRIPT_DIR/../qbase_runtime/frontend/shared/
cargo test export_bindings
rm $TARGET_DIR/* -rf
mv $SRC_DIR/* $TARGET_DIR
