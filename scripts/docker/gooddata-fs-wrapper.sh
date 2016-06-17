#! /usr/bin/env sh

uid=$1
gid=$2
shift 2

groupadd -g $gid user
useradd -d /rust -u $uid -g $gid -G fuse user
sudo -u user ./bin/gooddata-fs $@
