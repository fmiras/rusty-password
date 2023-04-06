#!/bin/bash

# Set the release tag to download
VERSION=$(curl --silent "https://api.github.com/repos/fmiras/rusty-password/releases/latest" | grep -Eo '"tag_name": "[^"]+"' | cut -d'"' -f4)

# Download the binary
URL="https://github.com/fmiras/rusty-password/releases/download/$VERSION/rusty-password"
curl -LO "$URL"

# Install the binary
sudo cp rusty-password /usr/local/bin/rusty-password

# Add to PATH
if [ -f ~/.bashrc ]; then
  echo "export PATH=$PATH:/usr/local/bin" >> ~/.bashrc
elif [ -f ~/.zshrc ]; then
  echo "export PATH=$PATH:/usr/local/bin" >> ~/.zshrc
fi

sudo chmod +x /usr/local/bin/rusty-password

# Cleanup
rm -f rusty-password
