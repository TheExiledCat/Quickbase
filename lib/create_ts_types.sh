SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cargo test export_bindings
mkdir -p $SCRIPT_DIR/../qbase_runtime/webui/src/classes/
rm $SCRIPT_DIR/../qbase_runtime/webui/src/classes/* -rf
mv bindings/* $SCRIPT_DIR/../qbase_runtime/webui/src/classes
