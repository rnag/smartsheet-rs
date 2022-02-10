# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

## v0.3.0 (2022-02-10)

### Features

- Trim down and refactor on the project dependencies.
- Make `hyper-rustls` the default TLS implementation, instead of `hyper-tls`. This should
  work out better for Linux deployments, for example on [AWS Lambda].
- Add optional and default features.
- Make the library logging opt-in.
- Add an implementation for `CellGetter::name_to_cell` method.
- Refactor `ParamBuilder` usage so it conforms a little better to best practices in Rust.

[AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

### Breaking Changes

- Comment out the `CellGetter::from_name_to_id` method, since I was fighting
   too long with the Rust compiler to make it work. I unfortunately ended up losing the battle there.

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
