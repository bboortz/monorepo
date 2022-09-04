#!/bin/bash

set -e 
set -u

USER_AGENT="Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0"
err_count=0

while read f; do
  echo -ne "** $f  --> "
  resp_code=$( curl -o /dev/null --no-progress-meter -w "%{http_code}\n" -L $f || echo $?)
  # resp_code=$( curl -o /dev/null --no-progress-meter -w "%{http_code}\n" -A "{$USER_AGENT}" $f || echo $?)

  if [ "$resp_code" != "200" ]; then
      let "err_count = err_count + 1"
      echo " *** ERROR: $resp_code ***"
  else
      echo " OK"
  fi
done < <( cat bookmarks.md| gawk 'match($0, /^* \[(.*)\]\((.*)\)/, a) {print a[2]}')

echo "NUMBER_OF_ERRORS: $err_count"
exit $err_count
