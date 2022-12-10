#!/usr/bin/env bash

# takes the tag as an argument (e.g. v0.1.0)
if [ -n "$1" ]; then
  if [ $1 == "init" ]; then
    if [ -n "$2" ]; then
      # update the Cargo.toml version of the rapiddb workspaces
      msg="# managed by release.sh"
      sed "s/^version = .* $msg$/version = \"${2#v}\" $msg/" -i rapiddb/Cargo.toml
      
      git-cliff --tag "$2" > CHANGELOG.md

      changelog=$(git-cliff --tag "$2" --strip all)

      git add -A && git commit -m "chore(release): prepare for $2"

      git tag -a "$2" -m "# Release $2" -m "$changelog"
	    git tag -v "$2"
    else
      echo "warn: please provide a tag"
    fi
  else
    update the Cargo.toml version of the rapiddb workspaces
    msg="# managed by release.sh"
    sed "s/^version = .* $msg$/version = \"${1#v}\" $msg/" -i rapiddb/Cargo.toml
    
    git-cliff --unreleased --tag "$1" --prepend CHANGELOG.md

    changelog=$(git-cliff --tag "$1" --strip all)

    git add -A && git commit -m "chore(release): prepare for $1"

    git tag -a "$1" -m "# Release $1" -m "$changelog"
    git tag -v "$1"
  fi
else
	echo "warn: please provide a tag"
fi
