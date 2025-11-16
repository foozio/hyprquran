#!/usr/bin/env bash
set -euo pipefail
docker compose up -d arch_builder
docker compose exec -u builder arch_builder bash -lc "rm -rf pkg src && makepkg -sf --noconfirm"
