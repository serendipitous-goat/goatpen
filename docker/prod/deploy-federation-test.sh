#!/bin/bash
set -e

# its a good idea to comment out lto in cargo.toml when using this to speed up the build
TAG="federation-test"

sudo docker build ../../ --file Dockerfile -t "dessalines/lemmy:$TAG"
sudo docker save "dessalines/lemmy:$TAG" -o "$TAG.tar"
sudo chown "$(id -u):$(id -g)" "$TAG.tar"

scp "$TAG.tar" enterprise.lemmy.ml:
rm "$TAG.tar"
ssh lemmy-test "cat $TAG.tar | docker load"
ssh lemmy-test "rm $TAG.tar"
ssh lemmy-test "cd /lemmy/enterprise.lemmy.ml && docker-compose up -d"