#!/usr/bin/env bash

set -e

location=$1
static_content=$(find ${location} -type f | grep -v *.html)

for file in $static_content ; do
    echo "start: $file"
    gzip --keep --best "$file"
    echo "done: $file"
done
