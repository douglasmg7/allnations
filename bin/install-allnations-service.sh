#! /usr/bin/env bash

# # Should run as a root.
# if [ "$EUID" -ne 0 ]; then 
  # echo "Please run as root"
  # exit
# fi

# CARGOPATH must be defined.
[[ -z "$CARGOPATH" ]] && printf "error: CARGOPATH enviorment not defined.\n" >&2 && exit 1 

# GS must be defined.
[[ -z "$GS" ]] && printf "error: GS enviorment not defined.\n" >&2 && exit 1 

# ZUNKAPATH must be defined.
[[ -z "$ZUNKAPATH" ]] && printf "error: ZUNKAPATH enviorment not defined.\n" >&2 && exit 1 

# ALLNATIONS_DB must be defined.
[[ -z "$ALLNATIONS_DB" ]] && printf "error: ALLNATIONS_DB enviorment not defined.\n" >&2 && exit 1 

# ALLNATIONS_USER must be defined.
[[ -z "$ALLNATIONS_USER" ]] && printf "error: ALLNATIONS_USER enviorment not defined.\n" >&2 && exit 1 

# ALLNATIONS_PASS must be defined.
[[ -z "$ALLNATIONS_PASS" ]] && printf "error: ALLNATIONS_PASS enviorment not defined.\n" >&2 && exit 1 

# Script not exist.
[[ ! -f $GS/allnations/bin/fetch-xml-products-and-process.sh ]] && printf "error: script $GS/allnations/bin/fetch-xml-products-and-process.sh not exist.\n" >&2 && exit 1 

# Uninstall script not exist.
[[ ! -f $GS/allnations/bin/uninstall-allnations-service.sh ]] && printf "error: script $GS/allnations/bin/uninstall-allnations-service.sh not exist.\n" >&2 && exit 1

# Remove allnations timer and allnations service.
$GS/allnations/bin/uninstall-allnations-service.sh

# Create log dir.
mkdir -p $ZUNKAPATH/log/allnations

# Make allnations script wide system accessible.
echo Creating symobolic link for allnations script...
sudo ln -s $CARGOPATH/bin/allnations /usr/local/bin/allnations

# Create aldo timer.
echo "creating '/lib/systemd/system/allnations.timer'..."
sudo bash -c 'cat << EOF > /lib/systemd/system/allnations.timer
[Unit]
Description=allnations timer

[Timer]
OnCalendar=*-*-* 00:00:01
OnCalendar=*-*-* 01:00:00
OnCalendar=*-*-* 02:00:00
OnCalendar=*-*-* 03:00:00
OnCalendar=*-*-* 04:00:00
OnCalendar=*-*-* 05:00:00
OnCalendar=*-*-* 06:00:00
OnCalendar=*-*-* 07:00:00
OnCalendar=*-*-* 08:00:00
OnCalendar=*-*-* 09:00:00
OnCalendar=*-*-* 10:00:00
OnCalendar=*-*-* 11:00:00
OnCalendar=*-*-* 12:00:00
OnCalendar=*-*-* 13:00:00
OnCalendar=*-*-* 14:00:00
OnCalendar=*-*-* 15:00:00
OnCalendar=*-*-* 16:00:00
OnCalendar=*-*-* 17:00:00
OnCalendar=*-*-* 18:00:00
OnCalendar=*-*-* 19:00:00
OnCalendar=*-*-* 20:00:00
OnCalendar=*-*-* 21:00:00
OnCalendar=*-*-* 22:00:00
OnCalendar=*-*-* 23:00:00

Persistent=true

[Install]
WantedBy=timers.target
EOF'

# Create aldo service.
echo "creating '/lib/systemd/system/allnations.service'..."
sudo GS=$GS ZUNKAPATH=$ZUNKAPATH ALLNATIONS_DB=$ALLNATIONS_DB ALLNATIONS_USER=$ALLNATIONS_USER ALLNATIONS_PASS=$ALLNATIONS_PASS bash -c 'cat << EOF > /lib/systemd/system/allnations.service
[Unit]
Description=allnations

[Service]
Type=oneshot
User=douglasmg7
Environment="GS=$GS"
Environment="ZUNKAPATH=$ZUNKAPATH"
Environment="ALLNATIONS_DB=$ALLNATIONS_DB"
Environment="ALLNATIONS_USER=$ALLNATIONS_USER"
Environment="ALLNATIONS_PASS=$ALLNATIONS_PASS"
Environment="RUN_MODE=production"
ExecStart=$GS/allnations/bin/fetch-xml-products-and-process.sh
EOF'

sudo systemctl start allnations.timer
sudo systemctl enable allnations.timer
