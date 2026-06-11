#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "========================================="
echo "   Building janalyze in Release Mode     "
echo "========================================="

# 1. Compile the optimized release binary
cargo build --release

echo ""
echo "========================================="
echo "   Installing system-wide (/usr/local/bin) "
echo "========================================="

# 2. Check if we have root privileges to write to /usr/local/bin
if [ "$EUID" -ne 0 ]; then
    echo "Elevated permissions required to install to /usr/local/bin."
    echo "Switching to sudo..."
    sudo cp target/release/janalyze /usr/local/bin/
else
    cp target/release/janalyze /usr/local/bin/
fi

# 3. Ensure the binary has executable permissions
sudo chmod +x /usr/local/bin/janalyze

echo ""
echo "Success! janalyze has been installed for all users."
echo "You can now run 'janalyze help' anywhere."
