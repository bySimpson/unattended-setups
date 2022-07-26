#!/usr/bin/env bash
sudo apt update && sudo apt install zsh git wget -y
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
cd ~/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-syntax-highlighting && cd ~
sed -i "s/ZSH_THEME=.*/ZSH_THEME=\"risto\"/g" ~/.zshrc
sed -i "s/plugins=.*/plugins=(git sudo zsh-syntax-highlighting git dirhistory extract web-search yum git-extras docker vagrant)/g" ~/.zshrc
sed -i '/unsetopt BEEP/d' ~/.zshrc && echo "unsetopt BEEP" >> ~.zshrc
exec /bin/zsh