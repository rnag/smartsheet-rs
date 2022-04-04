# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

## v0.6.0 (2022-04-04)

### Breaking Changes

* Update model struct name: `Attachment` -> `AttachmentMeta`

### Features

* Add support for the following API methods:
  - [List Attachments](https://smartsheet-platform.github.io/api-docs/#list-attachments)
  - [Get Attachment](https://smartsheet-platform.github.io/api-docs/#get-attachment)
* Add method `SmartsheetApi::list_attachments`
* Add method `SmartsheetApi::get_attachment`

## v0.5.0 (2022-02-27)

### Features

* Add support for the following API methods:
  - [Add Rows](https://smartsheet-platform.github.io/api-docs/#add-rows)
  - [Update Rows](https://smartsheet-platform.github.io/api-docs/#update-rows)
  - [Delete Rows](https://smartsheet-platform.github.io/api-docs/#delete-rows)
* Add helper struct `RowGetter` to find row(s) in a sheet that match a
  specified condition.
* Add helper struct `CellFactory` which makes it easier to create new `Cell`s,
  useful when *adding* or *updating* rows in a sheet.
* Add fluent methods to `Row`
* Add convenience methods to `Row` such as `Row::with_cells`
* Add new convenience method `SmartsheetApi::get_sheet_with_multi_contact_info`
  * Also add an example `cell_multi_contact`, which demonstrates how to work with
    `MULTI_CONTACT` cell types in smartsheet.
* Add method `Sheet::get_row_by_id`
* Add method `Sheet::id_to_row`
* Add `CellValue::from` implementations, so we can more easily work with standard
  Rust types such as `&str`.
* Add `ColumnMapper::from` implementations, so we can more easily create it
  from a `Sheet` object, for example.

## v0.4.0 (2022-02-25)

### Features

* Add `level` parameter to `get_sheet_with_params`, so it's possible to retrieve
  the emails for `MULTI_CONTACT` cells, for example.
* Change parameter definitions that use `Option<T>` to use `impl Into<Option<T>>` instead, so it's possible
  for the user to just specify a `T` as an input.
* Update badges used in the docs.

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

## v0.1.0 (2022-02-06)

- Initial Release on [crates.io][] :tada:

[crates.io]: https://crates.io/crates/smartsheet-rs
