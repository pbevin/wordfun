# List available recipes
default:
    @just --list

# Deploy the current commit to Fly.io and check it went live
deploy:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "$(git status --porcelain)" ]; then
        echo "Working tree has uncommitted changes; commit or stash them first." >&2
        exit 1
    fi
    commit=$(git rev-parse HEAD)
    fly deploy --build-arg SOURCE_COMMIT="$commit"
    live=$(curl -fsS https://wordfun.ca/version.txt)
    if [ "$live" = "$commit" ]; then
        echo "wordfun.ca is now running $commit"
    else
        echo "wordfun.ca/version.txt shows '$live', expected $commit" >&2
        exit 1
    fi
