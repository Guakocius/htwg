#!/bin/sh

set -e

readarray -t rec < mails.txt
msg=""
mail_body=""
sbj=""

RESET=$'\e[0m'
BOLD=$'\e[1m'
GREEN=$'\e[32m'

while getopts "hs:m:" flag; do
  case $flag in
    h) 
cat << EOF
usage: $0 -s  [Message's Subject] -m [Message being sent]
-h            Show this help message
-s            Message's Subject
-m            Message being sent
EOF
exit
;;
    s) 
      sbj="$OPTARG" 
      ;;
    m) 
      msg="$OPTARG" 
      ;;
    \?)
      ;;
  esac
done

FILE="$(< ./test.txt)"
for i in ${rec[@]}; do
  mail -s "$sbj" "$i" <<< "$FILE" 
  echo "Email to ${BOLD}$i${RESET} sent ${GREEN}successfully!${RESET}"
done



