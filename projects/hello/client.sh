#!/bin/sh

if [ $# -ne 2 ] ; then
	echo "Usage: ./client.sh <project name>"
	exit
fi
project=$1
port=$2
while [ $(ps -a | grep $project | wc -l) -eq 0 ] ; do
	sleep 1
done
sleep 1
curl localhost:$port
sleep 1
if [ $(ps -a | grep $project | wc -l) -eq 1 ] ; then
	kill $(ps -a | grep $project | awk '{print $1}')
fi

