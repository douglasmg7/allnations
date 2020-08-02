#!/usr/bin/env bash

# ZUNKAPATH must be defined.
[[ -z "$ZUNKAPATH" ]] && printf "error: ZUNKAPATH enviorment not defined.\n" >&2 && exit 1 

# Go to source path.
cd $(dirname $0)
cd ..

# Last downloaded XML file.
FILE=$ZUNKAPATH/xml/allnations/allnations-products.xml

# if [[ $RUN_MODE == production ]]; then
if [[ $1 == "-p" ]]; then
    allnations -p < $FILE
else
    cargo run < $FILE
fi
