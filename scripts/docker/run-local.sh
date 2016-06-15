#! /usr/bin/env sh

if [ $# -ge 3 ]; then
    host_mountpoint=$3
else
    host_mountpoint=gd-fs
fi

docker run -i -t --privileged=true --rm -v "$host_mountpoint":/gd-fs gooddata-fs $1 $2 /gd-fs
