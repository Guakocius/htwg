#!/bin/sh
set -euo pipefail

# NOTE: Needed Packages: msmtp mailx ca-certificates

CONFIG_DIR="$HOME/.config/mail"
MSMTP_CONFIG="$HOME/.msmtprc"

echo "=== HTWG SOGo Mail Init ==="
echo

read -rp "HTWG email address: " EMAIL
read -rp "HTWG user name: " USER
#read -rsp "HTWG email password: " PASSWORD
echo

mkdir -p "$CONFIG_DIR"

# -------------------------
# msmtp configuration
# -------------------------
cat > "$MSMTP_CONFIG" <<EOF
defaults
auth           on
tls            on
tls_trust_file /etc/ssl/certs/ca-certificates.crt
logfile        $CONFIG_DIR/msmtp.log

account sogo
host smtp.htwg-konstanz.de
port 587
from $EMAIL
user $USER
passwordeval "pass show sogo/mail"

account default : sogo
EOF

chmod 600 "$MSMTP_CONFIG"

# -------------------------
# mailx configuration
# -------------------------
cat > "$HOME/.mailrc" <<EOF
set mta="/usr/bin/msmtp"
set message-sendmail-extra-arguments="-a htwg"
set from="$EMAIL"
EOF

echo
echo "✔ Mail configuration installed"
echo "✔ Config file: ~/.msmtprc"
echo "✔ Mail backend: msmtp"
echo
echo "Test with:"
echo '  echo "Hello" | mail -s "Test Mail" you@example.com'

