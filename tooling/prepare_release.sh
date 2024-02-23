#!/usr/bin/env bash

# Example
# ```sh
# ./prepare_release.sh v0.1.18
# ```

set -Eeuo pipefail

ci () {
  cargo +nightly fmt --all
  cargo clippy --all-targets --all-features -- -Dwarnings
  cargo test
}

bump_version () {
  # update the Cargo.toml version of the rapiddb workspaces
  msg="# prepare_release.sh"

  sed "s/^version = .* $msg$/version = \"${1#v}\" $msg/" -i rapiddb/Cargo.toml
  sed "s/^version = .* $msg$/version = \"${1#v}\" $msg/" -i rapiddb-web/Cargo.toml

  cargo check
}

prepare_tag () {
  # generate a changelog for the tag message
  export GIT_CLIFF_TEMPLATE="\
  {% for group, commits in commits | group_by(attribute=\"group\") %}
  {{ group | upper_first }}\
  {% for commit in commits %}
    - {% if commit.breaking %}(breaking) {% endif %}{{ commit.message | upper_first }} ({{ commit.id | truncate(length=7, end=\"\") }})\
  {% endfor %}
  {% endfor %}"
  changelog=$(git-cliff --config tooling/git-cliff-detailed.toml --unreleased --strip all)
  
  git add -A && git commit -m "chore(release): prepare for $1"

  git fetch --all --tags
  git tag -a "$1" -m "# Release $1" -m "$changelog"
  git tag -v "$1"
}

# takes the tag as an argument (e.g. v0.1.0)
if [ -n $1 ]; then
  if [ $1 == "init" ]; then
    if [ -n $2 ]; then
      ci

      bump_version $2
      
      git-cliff --config tooling/cliff.toml --tag "$2" > CHANGELOG.md

      prepare_tag $2
    else
      echo "warn: please provide a tag"
    fi
  else
    ci
    
    bump_version $1
    
    git-cliff --config tooling/cliff.toml --unreleased --tag "$1" --prepend CHANGELOG.md

    prepare_tag $1
  fi
else
	echo "warn: please provide a tag"
fi
