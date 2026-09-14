#!/bin/sh -e

EX_USAGE=64

print_usage() {
    echo 'Usage: gen-pac.sh MCU_NAME' >&2
}

if [ $# -ne 1 ]; then
    print_usage
    exit $EX_USAGE
fi

mcu_name=$1
BUILD_DIR=${BUILD_DIR:-build}
pac_path=$BUILD_DIR/pacs/${mcu_name}-lpa

# Create a clean directory for the PAC.

rm -rf "$pac_path"
mkdir -p "$pac_path"

# Generate the crate metadata from the templates.

find pac-template -mindepth 1 | while IFS= read -r path; do
    rel_path=${path#pac-template/}

    if [ -d "$path" ]; then
        mkdir -p "$pac_path/$rel_path"
    elif [ "${rel_path%.jinja}" != "$rel_path" ]; then
        rel_path=${rel_path%.jinja}
        jinja2 --strict -o "$pac_path/$rel_path" "$path" "pac-metadata/$mcu_name.toml"
    else
        cp "$path" "$pac_path/$rel_path"
    fi
done

# Generate the crate source code from the SVD file.

svd2pac "build/svds/${mcu_name}.svd" "$pac_path" --license-file LICENSE

cd "$pac_path"
cargo check