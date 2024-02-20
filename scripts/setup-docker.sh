#!/usr/bin/env bash
sudo apt update && sudo apt install curl -y
sh -c "$(curl -fsSL https://get.docker.com)"
sudo apt-get install docker-compose-plugin -y
sudo usermod -aG docker $USER
sudo service docker start
