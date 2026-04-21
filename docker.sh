#!/usr/bin/env bash
set -euo pipefail

CONTAINER_NAME="${CONTAINER_NAME:-rcore-tutorial-v3}"
IMAGE_NAME="${IMAGE_NAME:-rcore-tutorial-v3:latest}"
WORKDIR="${WORKDIR:-/mnt}"
HOST_DIR="${HOST_DIR:-$(pwd)}"

if docker ps -a --format '{{.Names}}' | grep -Fxq "${CONTAINER_NAME}"; then
  if docker ps --format '{{.Names}}' | grep -Fxq "${CONTAINER_NAME}"; then
    exec docker exec -it "${CONTAINER_NAME}" bash
  else
    docker start "${CONTAINER_NAME}" >/dev/null
    exec docker exec -it "${CONTAINER_NAME}" bash
  fi
fi

exec docker run --rm -it \
  -v "${HOST_DIR}:${WORKDIR}" \
  -w "${WORKDIR}" \
  --name "${CONTAINER_NAME}" \
  "${IMAGE_NAME}" \
  bash
