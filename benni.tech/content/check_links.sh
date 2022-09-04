#!/bin/bash

set -e 
set -u

USER_AGENT="Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0"
err_count=0

check_link() {
  local f="$1"

  # echo -ne "** $f  --> "
  resp_code=$( curl -o /dev/null --no-progress-meter -w "%{http_code}\n" -L --connect-timeout 3 --max-time 10 $f || echo $?)
  # resp_code=$( curl -o /dev/null --no-progress-meter -w "%{http_code}\n" -A "{$USER_AGENT}" $f || echo $?)

  if [ "$resp_code" != "200" ]; then
      let "err_count = err_count + 1"
      echo "** $f  -->  *** ERROR: $resp_code ***"
      #echo " *** ERROR: $resp_code ***"
  else
      echo "** $f  -->  OK"
      # echo " OK"
  fi
}

has_duplicates()
{
  {
    sort | uniq -d | grep . -qc
  } < "$1"
}

check_dup() {
  local f="$1"
  if has_duplicates <( cat $f | gawk 'match($0, /^* \[(.*)\]\((.*)\)/, a) {print a[2]}') ; then
    echo "** file $f has duplicated lines."
    cat $f | gawk 'match($0, /^* \[(.*)\]\((.*)\)/, a) {print a[2]}' | sort | uniq -d
    exit 1
  else
    echo "** file $f has no duplicated line."
  fi
}


check_dup bookmarks.md

while read f; do
  check_link "$f" &
  sleep 0.1
done < <( cat bookmarks.md| gawk 'match($0, /^* \[(.*)\]\((.*)\)/, a) {print a[2]}')

while true; do
  if [ $(jobs | grep Running | wc -l) -eq 0 ]; then
    break 
  fi
  sleep 1
done

echo "NUMBER_OF_ERRORS: $err_count"
exit $err_count
