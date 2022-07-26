#!/usr/bin/env bash
sudo apt update && sudo apt install bat -y
if test -f "/usr/bin/bat"; then
    sudo rm /usr/bin/bat
fi
sudo ln -s /usr/bin/batcat /usr/bin/bat
