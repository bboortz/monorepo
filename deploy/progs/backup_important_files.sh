#!/bin/bash

#set -e
set -u

DATE="$( date +%Y-%m-%d-%H-%M )"
HOME=/home/benni
SRC_DIR="/run/media/benni/samba/benni/important"
DST_WEBDAV="/run/media/benni/webdav/Benjamin Boortz/Documents"
DST_STICK1="/run/media/benni/SanDisk"
DST_STICK2="/run/media/benni/0ba9f7e4-d736-4080-9c6b-c6680c7cf248"

echo "*** BACKUP IMPORTANT FILES ***"
echo "* DATE: $DATE"
echo
echo "* SRC_DIR:    $SRC_DIR"
echo "* DST_WEBDAV: $DST_WEBDAV"
echo "* DST_STICK1: $DST_STICK1"
echo "* DST_STICK2: $DST_STICK2"
echo

touch "${SRC_DIR}/LAST_UPDATE_FROM_T540P"
ls -la "${SRC_DIR}/LAST_UPDATE_FROM_T540P"


echo
echo "* $SRC_DIR  -->  $DST_WEBDAV"
if [ -f "${DST_WEBDAV}/TESTFILE" ]; then
    rsync -av --timeout=3 --bwlimit=5000 ${SRC_DIR}/* "${DST_WEBDAV}/"
    ls -la "${DST_WEBDAV}/LAST_UPDATE_FROM_T540P"
fi


echo
echo "* $SRC_DIR  -->  $DST_STICK1"
if [ -f "${DST_STICK1}/TESTFILE" ]; then
    mkdir -p "${DST_STICK1}/Backup/${DATE}"
    rsync -av --exclude="Backup" ${DST_STICK1}/* "${DST_STICK1}/BACKUP/${DATE}"
    rsync -av ${SRC_DIR}/* "${DST_STICK1}/"
    ls -la "${DST_STICK1}/LAST_UPDATE_FROM_T540P"


    echo
    echo "* $DST_STICK1  -->  $DST_STICK2"
    if [ -f "${DST_STICK2}/TESTFILE" ]; then
        rsync -av ${DST_STICK1}/* "${DST_STICK2}/"
        ls -la "${DST_STICK2}/LAST_UPDATE_FROM_T540P"
    fi


    echo
    echo "* CLEANUP $DST_STICK1"
    for i in {1..30}; do i2=$((i+1)); find "${DST_STICK1}/Backup" -maxdepth 1 -newermt "-${i2} days" -not -newermt "-${i} days" | head -n -1 | xargs rm -r ;done

fi


