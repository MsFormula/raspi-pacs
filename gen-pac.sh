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

find pac-template -exec sh -ec "
    path=\$1

    path_no_pac_template=\${path#pac-template/}
    if [ \"\$path_no_pac_template\" = \"\$path\" ]; then
        exit
    fi
    path=\$path_no_pac_template

    if [ -d \"\$1\" ]; then
        mkdir -p \"$pac_path/\$path\"
    else
        path_no_jinja=\${path%.jinja}
        if [ \"\$path_no_jinja\" != \"\$path\" ]; then
            path=\$path_no_jinja
            jinja2 --strict -o \"$pac_path/\$path\" \"\$1\" \\
                \"pac-metadata/$mcu_name.toml\"
        else
            cp \"\$1\" \"$pac_path/\$path\"
        fi
    fi
" sh {} \;

# Generate the crate source code from the SVD file.

svd2rust -i "build/svds/${mcu_name}.svd" --target none --atomics \
    --atomics-feature atomic --impl_debug --impl-defmt defmt -o "$pac_path"

# Format the source code.

cd "$pac_path"
mkdir src
form -i lib.rs -o src/
rm lib.rs
cargo fmt
