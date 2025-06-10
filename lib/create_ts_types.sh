SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cargo test export_bindings
rm $SCRIPT_DIR/../qbase_runtime/webui/src/classes/* -r
mv bindings/* $SCRIPT_DIR/../qbase_runtime/webui/src/classes
