#!/usr/bin/env bash 

# ZUNKAPATH not defined.
if [ -z "$ZUNKAPATH" ]; then
	printf "error: ZUNKAPATH not defined.\n" >&2
	exit 1 
fi

# ALLNATIONS_DB not defined.
if [ -z "$ALLNATIONS_DB" ]; then
	printf "error: ALLNATIONS_DB not defined.\n" >&2
	exit 1 
fi

# Create db if not exist.
if [[ ! -f $ALLNATIONS_DB ]]; then
	echo Creating $ALLNATIONS_DB
    mkdir -p $ZUNKAPATH/db
    sqlite3 $ALLNATIONS_DB < $(dirname $0)/tables.sql
fi
