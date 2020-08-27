#!/usr/bin/env bash

# ZUNKAPATH must be defined.
[[ -z "$ZUNKAPATH" ]] && printf "error: ZUNKAPATH enviorment not defined.\n" >&2 && exit 1 

# Go to source path.
cd $(dirname $0)
cd ..

# Last downloaded XML file.
FILE=$ZUNKAPATH/xml/allnations/allnations_products_to_process.xml

if [ ! -f $FILE ]; then
    echo No file $FILE to process.
    exit 0
fi

if [[ $RUN_MODE == production ]]; then
    RUN_MODE=production allnations < $FILE | tee -a $ZUNKAPATH/log/allnations/allnations.log
else
    cargo run < $FILE | tee -a $ZUNKAPATH/log/allnations/allnations.log 
fi

# Remove file if successful processed.
[[ $? == 0 ]] && rm $FILE
