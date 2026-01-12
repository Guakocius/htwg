#!/usr/bin/env bash

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

# NOTE: Both FILE as well as CID should be manually assignable as flags
FILE="$(< ./test.txt)"
CID="schiller"
BOUNDARY="BOUNDARY_$(date +%s)"
for i in ${rec[@]}; do
  #mail -s "$sbj" "$i" <<< "$FILE"
  {
    # RFC-5322 Headers
    echo "From: alexander.engelhardt@htwg-konstanz.de";
    echo "To: $i";
    echo "Subject: Test Image";
    # MIME Body
    echo "MIME-Version: 1.0";
    echo "Content-Type: multipart/related; boundary=\"$BOUNDARY\"";
    echo;
    echo "--$BOUNDARY";
    echo "Content-Type: text/html; charset=UTF-8";
    echo
  cat <<EOF
<html>
  <body>
    <h2>Hello Header Lorem ipsum whatever</h2>
    <p>Example paragraph image</p> &#10084;&#65039;
    <img src="cid:$CID" alt="Schiller being meme-y">
  </body>
</html>
EOF
  echo;
  echo "--$BOUNDARY";
  echo "Content-Type: image/jpeg";
  echo "Content-ID: <$CID>";
  echo "Content-Disposition: inline; filename=\"schiller_zitat.jpeg\"";
  echo "Content-Transfer-Encoding: base64";
  echo;
  base64 schiller_zitat.jpeg;
  echo;
  echo "--$BOUNDARY--";
  } | msmtp "$i" 
  echo "Email to ${BOLD}$i${RESET} sent ${GREEN}successfully!${RESET}"
done
