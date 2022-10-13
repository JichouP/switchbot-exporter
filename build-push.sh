#!/bin/bash

if [ $# != 1 ]; then
  echo "invalid args"
  echo "example: ./build-push 0.1.0"
  exit 1
fi

# docker buildx create --name mybuilder --use
docker buildx build --platform linux/amd64,linux/arm64 -t jichoup/switchbot-exporter:$1 -t jichoup/switchbot-exporter:latest --push .
