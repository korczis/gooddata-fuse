#! /usr/bin/env sh

if [ $# -ge 3 ]; then
    host_mountpoint=$3
else
    host_mountpoint=gd-fs
fi

host_mountpoint_parent=$(df -h --output=target "$host_mountpoint" | tail -n1)
host_mountpoint_sharing=$(findmnt -n -o PROPAGATION "$host_mountpoint_parent")

if [ "$host_mountpoint_sharing" != shared ]; then

    docker run -i --privileged=true --entrypoint chroot \
        --rm -v /:/host-root gooddata-fs \
        /host-root/ nsenter -t 1 -m mount --make-shared "$host_mountpoint_parent"
fi

docker run -i -t --privileged=true --rm -v "$host_mountpoint":/gd-fs:shared gooddata-fs \
    $(id -u) $(id -g) $1 $2 /gd-fs
