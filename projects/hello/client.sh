#!/bin/sh

if [ $# -ne 1 ] ; then
	echo "Usage: ./client.sh <project name>"
	exit
fi
project=$1
while [ $(ps -a | grep $project | wc -l) -eq 0 ] ; do
	sleep 1
done

