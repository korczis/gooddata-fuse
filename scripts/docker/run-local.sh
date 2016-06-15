#! /usr/bin/env sh

docker run -i -t --privileged=true --rm -v gd-fs:/gd-fs:ro gooddata-fs
