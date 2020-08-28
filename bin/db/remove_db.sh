#!/usr/bin/env bash 

# ZUNKAPATH not defined.
if [ -z "$ZUNKAPATH" ]; then
	printf "error: ZUNKAPATH not defined.\n" >&2
	exit 1 
fi

# ZUNKA_ALLNATIONS_DB not defined.
if [ -z "$ALLNATIONS_DB" ]; then
	printf "error: ALLNATIONS_DB not defined.\n" >&2
	exit 1 
fi

printf "Removing db %s/%s\n" $ALLNATIONS_DB
rm $ALLNATIONS_DB
