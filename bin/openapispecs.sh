#!/usr/bin/env sh
#-*-mode: Shell-script; coding: utf-8;-*-
# SPDX-License-Identifier: BlueOak-1.0.0
# Description:
_base=$(basename "$0")
_dir=$(cd -P -- "$(dirname -- "$(command -v -- "$0")")" && pwd -P || exit 126)
export _base _dir
set "${SETOPTS:--eu}"

# Version of open-webui we are connecting to, TODO snag this dynamically?
VERSION="${VERSION:-0.3.35}"
HOST="${HOST?You need to set HOST to an open-webui system, no auth for now}"
install -dm755 openapi

destver="openapi/${VERSION}"
install -dm755 "${destver}"

# Pipe the openapi through jq to format it, technically not needed

# Note this beast is not just one api but many
# ollama api
ollamadir="${destver}/ollama"
install -dm755 "${ollamadir}"
curl -s "${HOST}/ollama/openapi.json" -o - | jq > "${ollamadir}/openapi.json"

# rag api
ragdir="${destver}/rag"
install -dm755 "${ragdir}"
curl -s "${HOST}/retrieval/api/v1/openapi.json" -o - | jq > "${ragdir}/openapi.json"

# webui api
webuidir="${destver}/webui"
install -dm755 "${webuidir}"
curl -s "${HOST}/api/v1/openapi.json" -o - | jq > "${webuidir}/openapi.json"

# default api? no idea what to call this
defdir="${destver}/default"
install -dm755 "${defdir}"
curl -s "${HOST}/openapi.json" -o - | jq > "${defdir}/openapi.json"
