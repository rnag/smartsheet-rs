# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

## v0.2.0 (2022-02-08)

### Features

- Add new supported API methods to client
  - `get_column`, `get_column_with_params`
  - `get_sheet_by_name`
  - `get_column_by_title`
- Update to support more query parameters, like filter a `get_sheet`
  response by a list of `column_ids`
- Add some new *examples*
- Add some new files
  - `CHANGELOG.md` - let's document our changes!
  - `CONTRIBUTING.md` - I have docs on how to contribute!
- Add `version-sync` dependency for dev, so we know if we forget to bump
  the package version in the `README.md` file!

### Breaking Changes

- Update few methods like `CellGetter.by_name` to return
  a `Result` instead of `Option`.

## v0.1.0 (2022-01-01)

- Initial Release on [crates.io][] :tada:

[crates.io]: https://crates.io/crates/smartsheet-rs
