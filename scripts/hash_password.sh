#!/bin/bash
#
# Generate Argon2 password hash for admin dashboard
# Requires: python3 with argon2-cffi package
#
# Usage:
#   ./scripts/hash_password.sh
#   ./scripts/hash_password.sh "your-password"
#
# Install dependency:
#   pip install argon2-cffi
#

set -e

# Check if argon2 is available
if ! python3 -c "import argon2" 2>/dev/null; then
    echo "Error: argon2-cffi not installed"
    echo "Install with: pip install argon2-cffi"
    exit 1
fi

# Get password
if [ -n "$1" ]; then
    PASSWORD="$1"
else
    echo -n "Enter password: "
    read -s PASSWORD
    echo
fi

if [ -z "$PASSWORD" ]; then
    echo "Error: Password cannot be empty"
    exit 1
fi

# Generate hash using Python
python3 << EOF
from argon2 import PasswordHasher
ph = PasswordHasher()
hash = ph.hash("$PASSWORD")
print(hash)
EOF
