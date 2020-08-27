#! /usr/bin/env bash

# Remove link for all user access.
if [[ -f /usr/local/bin/allnations ]]; then
    echo Removing link for allnations...
    sudo rm /usr/local/bin/allnations
fi

# Remove allnation timer and allnation service.
if systemctl list-units --full --all | grep -Fq "allnations.timer"; then
    echo Removing service...
    sudo systemctl stop allnations.timer 
    sudo systemctl disable allnations.timer 
    sudo rm -v /lib/systemd/system/allnations.timer
    sudo rm -v /lib/systemd/system/allnations.service
    sudo systemctl daemon-reload
    sudo systemctl reset-failed
fi
