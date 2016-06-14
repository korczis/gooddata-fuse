#! /usr/bin/env sh

docker run -i -t --privileged=true --name gooddata-fs -v gd-fs:/gd-fs:ro gooddata-fs
