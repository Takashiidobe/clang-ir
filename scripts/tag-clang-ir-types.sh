#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/clang-ir-types/Cargo.toml"

version="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' "$manifest" | head -n1)"
if [[ -z "$version" ]]; then
  echo "Could not find version in $manifest" >&2
  exit 1
fi

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
  echo "Refusing to tag malformed clang-ir-types version: $version" >&2
  exit 1
fi

tag="${version}-clang-ir-types"

if git -C "$repo_root" rev-parse --verify --quiet "refs/tags/$tag" >/dev/null; then
  echo "Tag $tag already exists" >&2
  exit 1
fi

git -C "$repo_root" tag "$tag"
echo "Created tag $tag for clang-ir-types $version"
echo "Push it with: git push origin \"$tag\""
