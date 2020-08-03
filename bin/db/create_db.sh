#!/usr/bin/env bash 

# ZUNKAPATH not defined.
if [ -z "$ZUNKAPATH" ]; then
	printf "error: ZUNKAPATH not defined.\n" >&2
	exit 1 
fi

# ZUNKA_ALLNATIONS_DB not defined.
if [ -z "$ZUNKA_ALLNATIONS_DB" ]; then
	printf "error: ZUNKA_ALLNATIONS_DB not defined.\n" >&2
	exit 1 
fi

# Create db if not exist.
if [[ ! -f $ZUNKA_ALLNATIONS_DB ]]; then
	echo Creating $ZUNKA_ALLNATIONS_DB
    mkdir -p $ZUNKAPATH/db
    sqlite3 $ZUNKA_ALLNATIONS_DB < $(dirname $0)/tables.sql
fi
