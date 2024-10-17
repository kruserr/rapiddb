# Changelog

All notable changes to this project will be documented in this file.

## [0.1.32] - 2024-10-17

### Build

- Update deps
- Use rust 1.82
- Use rust 1.81

### Ci

- Update ci tooling

## [0.1.31] - 2024-05-26

### Ci

- Dont fail pipeline if codecov fails

## [0.1.30] - 2024-05-26

### Bug Fixes

- Clippy warnings

### Features

- Use proper error handling in mmav and mmav_unit

### Build

- Cargo update
- Use rust 1.78

## [0.1.29] - 2024-03-05

### Build

- Update deps to address CVE-2024-27308

## [0.1.28] - 2024-02-23

### Documentation

- Remove github releases shield

## [0.1.27] - 2024-02-23

### Ci

- Dont use v tags

## [0.1.26] - 2024-02-23

### Ci

- Run cd on semantic versioning
- Use semantic version changelog

## [0.1.25] - 2024-02-23

### Build

- Move files to tooling

### Ci

- Update to use Dockerfile in tooling and use docker cache tag
- Add missing \
- Update release tooling
- Login to github registry before pulling cache
- Use ghcr.io
- Use new docker build
- Add tooling/Dockerfile
- Use correct secrets
- Set constraints
- Use updated actions

## [0.1.24] - 2024-02-22

### Bug Fixes

- Update only-db example

### Documentation

- Remove content-type from curl

### Build

- Update to rust 1.76
- Update Cargo.lock
- Disable toolchain file
- Remove toolchain file

### Ci

- Set workspace lints

## [0.1.23] - 2024-01-23

### Documentation

- Add dockerhub readme
- Cleanup docs to include
- Dont use arc rwlock
- Remove README suffix
- Refactor
- Add dependencies to Cargo.toml
- Update docs
- Link to further install options for rest api
- Update docs
- Update
- Update
- Update

### Features

- [**breaking**] Expose endpoints_with_arc_rwlock and without to make api cleaner

### Build

- Update gitignore
- Cargo update
- Use rust 1.75

### Ci

- Add local ci script

## [0.1.22] - 2023-11-26

### Build

- Add version to rapiddb to fix cargo publish

## [0.1.21] - 2023-11-26

### Documentation

- #15: update documentation
- #15: update documentation
- #15: add example using the db without web

### Features

- #15: extract api into rapiddb-web
- #15: extract api into rapiddb-web
- #15: extract api into rapiddb-web
- #15: extract api into rapiddb-web
- #15: use rapiddb-web in docker container

### Styling

- Fmt

### Ci

- #15: publish rapiddb-web seperatly

## [0.1.20] - 2023-11-25

### Features

- #13: add IAsyncDatabase trait
- #13: basic MMAVAsyncDatabase implementation
- [**breaking**] #13: use IAsyncDatabase in warp

### Refactor

- #13: move mmav to subfolder
- #13: rename IDatabase file

### Build

- #13: add async-trait

## [0.1.19] - 2023-11-25

### Features

- #11: use static dispatch

## [0.1.18] - 2023-11-19

### Documentation

- Add example to prepare_release.sh

### Miscellaneous Tasks

- Update memmap2 requirement from 0.7 to 0.9 in /rapiddb

### Build

- Update Cargo.lock and use resolver 2
- Use rust 1.74

## [0.1.17] - 2023-06-20

### Documentation

- Remove duplicate changelog

### Miscellaneous Tasks

- Update memmap2 requirement from 0.5 to 0.7 in /rapiddb

### Build

- Cargo update

### Ci

- Use rust 1-70 and bookworm

## [0.1.16] - 2022-12-14

### Ci

- Only run publish-docker after ci succeeds
- Only keep logic for latest docker image
- Publish patch, minor and major versions

## [0.1.15] - 2022-12-14

### Documentation

- Update images and add docker img shield

### Ci

- Update ci and cd pipelines
- Change check to test
- Remove actions-rs/toolchain@v1 and add Swatinem/rust-cache@v2
- Run all jobs in parallel
- Dont cache rustfmt and publish to dockerhub
- Revert to old login method
- Fix login

## [0.1.14] - 2022-12-14

### Bug Fixes

- Refactor and run ci before tagging
- Use new # prepare_release.sh

### Documentation

- Add contributing guide
- Add pull request template
- Specify nightly fmt and stable clippy

### Refactor

- Apply clippy nightly fix
- Rename rustfmt
- Clippy fix

### Ci

- Update clippy command

## [0.1.13] - 2022-12-13

### Documentation

- Remove duplicate changelog entries

### Ci

- Attempt to fix duplicate changelog entries
- Attempt to fix duplicate changelog entries 2
- Attempt to fix duplicate changelog entries 3
- Attempt to fix duplicate changelog entries 4

## [0.1.12] - 2022-12-13

### Documentation

- Remoev duplicate entry in changelog

### Ci

- Dont cargo publish here
- Bump rapiddb version
- Run caro check after bumping version
- Publish monorepo rapiddb
- Run cargo build after vesion bump
- Run cargo check both places

## [0.1.11] - 2022-12-13

### Documentation

- Add logo
- #4: add test coverage, cargo link and logo
- Add releases and backlinks to github
- Update logo path and name and add favicon
- Change to use github hosted image and add img shield for docker hub for future docker hub deployment
- #4: add test coverage shield for crates.io and project banner to readme
- Open links in new tab
- Use github release latest semver for img shield

### Refactor

- Clippy fix

### Styling

- Fmt

### Ci

- #4: add Cargo.lock to .gitignore
- #4: update ci / cd pipelines
- Run fmt first and fail all if fmt fails

## [0.1.10] - 2022-12-10

### Refactor

- Fix clippy warnings

### Ci

- Run dependabot on rapiddb workspace
- Run lint
- Typo

## [0.1.9] - 2022-12-10

### Documentation

- Use less specific rapiddb version

## [0.1.8] - 2022-12-10

### Ci

- Remove git show
- Rename to publish and use cargo publish

## [0.1.7] - 2022-12-10

### Bug Fixes

- Dont sign tags

### Features

- #2: remove mariadb and redis integration
- Add release script

### Styling

- Change fmt max_width to 80
- Fmt

### Ci

- Add git-cliff

<!-- generated by git-cliff -->
