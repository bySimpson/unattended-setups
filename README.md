# unattended-setups
A set of unattended setup scripts for Debian-based Distros. With only one command the specified tool will be installed and configured!

# CLI installer
To use the interactive cli tool, use the following command:
```sh
sh -c "$(curl -fsSL https://thmr.at/cli)"
```

If you want to use a different release/branch, use the following command:
```sh
sh -c "$(curl -fsSL https://thmr.at/cli/{BRANCH/RELEASE})"
```

## Supported tools
| Tool    | Command                                                       |
|---------|---------------------------------------------------------------|
| BAT     | sh -c "$(curl -fsSL https://thmr.at/setup/bat)"               |
| Docker  | sh -c "$(curl -fsSL https://thmr.at/setup/docker)"            |
| dust    | sh -c "$(curl -fsSL https://thmr.at/setup/dust)"              |
| glow    | sh -c "$(curl -fsSL https://thmr.at/setup/glow)"              |
| ripgrep | sh -c "$(curl -fsSL https://thmr.at/setup/ripgrep)"           |
| ZSH     | sh -c "$(curl -fsSL https://thmr.at/setup/zsh)"               |
