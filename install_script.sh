#!/usr/bin/env bash
set -e

echo "Downloading the latest release of janalyze..."

# GitHub automatically redirects 'latest' to the newest uploaded binary
DOWNLOAD_URL="https://github.com/balint0513/janalyze/releases/latest/download/janalyze"

# Download the file to a temporary directory
curl -sSL -o /tmp/janalyze "$DOWNLOAD_URL"

echo "Installing to /usr/local/bin (this requires sudo privileges)..."

# Move it to the universal path and make it executable
sudo mv /tmp/janalyze /usr/local/bin/janalyze
sudo chmod +x /usr/local/bin/janalyze

echo ""
echo "Success! janalyze is installed."
echo "Type 'janalyze help' to get started."
