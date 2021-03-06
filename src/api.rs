//! Smartsheet API v2 implementation in Rust
//!
use crate::auth::auth_token;
use crate::builders::ParamBuilder;
use crate::constants::{API_ENDPOINT, ENV_VAR_NAME};
use crate::https::{get_https_client, tls};
use crate::log::{debug, warn};
use crate::models::*;
use crate::status::raise_for_status;
use crate::types::Result;
use crate::utils::*;

use std::io::{Error, ErrorKind};
use std::time::Instant;

use hyper::client::HttpConnector;
use hyper::header::AUTHORIZATION;
use hyper::{Body, Client, Method, Request};

/// Client implementation for making requests to the *Smartsheet
/// API v2*
///
/// # Links
/// - [`smartsheet-rs`](https://docs.rs/smartsheet-rs)
/// - [Official Documentation](https://smartsheet-platform.github.io/api-docs/)
///
pub struct SmartsheetApi<'a> {
    bearer_token: String,
    client: Client<tls::HttpsConnector<HttpConnector>>,
    endpoint: &'a str,
}

impl<'a> SmartsheetApi<'a> {
    /// Initialize a new `SmartsheetApi` object from an API access token.
    pub fn from_token(token: &str) -> Self {
        Self::new(API_ENDPOINT, token)
    }

    /// Initialize a new `SmartsheetApi` object from an API access token,
    /// assuming this is currently set in the environment.
    pub fn from_env() -> Result<Self> {
        let token: String = match std::env::var(ENV_VAR_NAME) {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "Environment variable `{name}` must be set.",
                    name = ENV_VAR_NAME
                ),
            )),
        }?;

        Ok(Self::new(API_ENDPOINT, &token))
    }

    /// Initialize a new `SmartsheetApi` object from a (custom) base API
    /// endpoint, and an access token.
    pub fn from_endpoint_and_token(endpoint: &'a str, token: &str) -> Self {
        Self::new(endpoint, token)
    }

    /// Constructor function, for internal use
    fn new(endpoint: &'a str, token: &str) -> Self {
        let bearer_token = auth_token(token);
        let client = get_https_client();

        Self {
            bearer_token,
            client,
            endpoint,
        }
    }

    /// **List Sheets** - Gets a list of all sheets that the user has access
    /// to in alphabetical order by name. The list contains an abbreviated
    /// Sheet object for each sheet.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-sheets
    ///
    pub async fn list_sheets(&self) -> Result<IndexResult<Sheet>> {
        self.list_sheets_with_params(None, None, None).await
    }

    /// **List Sheets** - Gets a list of all sheets that the user has access
    /// to in alphabetical order by name, with included _query parameters_.
    /// The list contains an abbreviated Sheet object for each sheet.
    ///
    /// # Arguments
    ///
    /// * `include` - A comma-separated list of elements to include in the response.
    /// * `include_all` - If true, include all results (i.e. do not paginate).
    /// * `modified_since` - Return sheets modified since a provided datetime.
    ///                      Date should be in ISO-8601 format, for example,
    ///                      `2020-01-30T13:25:32-07:00`.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-sheets
    ///
    pub async fn list_sheets_with_params(
        &self,
        include: impl Into<Option<Vec<ListSheetIncludeFlags>>>,
        include_all: impl Into<Option<bool>>,
        modified_since: impl Into<Option<&'a str>>, // TODO change this to a DATE type maybe
    ) -> Result<IndexResult<Sheet>> {
        let mut url = format!("{}/{}", self.endpoint, "sheets");

        ParamBuilder::new(&mut url)
            .with_comma_separated_values("include", include.into())
            .with_value("includeAll", include_all.into())
            .with_value("modifiedSince", modified_since.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        let sheets = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(sheets)
    }

    /// **Get Sheet** - Retrieves the specified sheet. Returns the sheet,
    /// including rows, and optionally populated with discussion and
    /// attachment objects.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows and data for.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-sheet
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    pub async fn get_sheet(&self, sheet_id: u64) -> Result<Sheet> {
        self.get_sheet_with_params(sheet_id, None, None, None, None, None, None, None)
            .await
    }

    /// **Get Sheet** - Retrieves the specified sheet. Returns the sheet,
    /// including rows, and optionally populated with discussion and
    /// attachment objects.
    ///
    /// # Note
    ///
    /// This is a convenience method to retrieve a Sheet with the `MULTI_CONTACT`
    /// cell data correctly populated. This is primarily important so that we can
    /// retrieve the email addresses for such cells, for example.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows and data for.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-sheet
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    pub async fn get_sheet_with_multi_contact_info(&self, sheet_id: u64) -> Result<Sheet> {
        self.get_sheet_with_params(
            sheet_id,
            // TODO: maybe change the underlying type to `slice` instead of `vec`?
            Some(vec![SheetIncludeFlags::Base(RowIncludeFlags::ObjectValue)]),
            None,
            None,
            None,
            None,
            None,
            Some(Level::MultiContact),
        )
        .await
    }

    /// **Get Sheet** - Retrieves the specified sheet, with included
    /// _query parameters_. Returns the sheet, including rows, and optionally
    /// populated with discussion and attachment objects.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows and data for.
    /// * `include` - A comma-separated list of elements to include in the response.
    /// * `exclude` - A comma-separated list of elements to _not_ include in the response.
    /// * `row_ids` - A comma-separated list of Row IDs on which to filter the
    ///               rows included in the result.
    /// * `row_numbers` - A comma-separated list of Row numbers on which to
    ///                   filter the rows included in the result. Non-existent
    ///                   row numbers are ignored.
    /// * `column_ids` - A comma-separated comma-separated list of Column IDs.
    ///                  The response will contain only the specified columns
    ///                  in the 'columns' array, and individual rows' 'cells'
    ///                  array will only contain cells in the specified columns.
    /// * `rows_modified_since` - Return rows modified since a provided datetime.
    ///                           Date should be in ISO-8601 format, for example,
    ///                           `2020-01-30T13:25:32-07:00`.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-sheet
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn get_sheet_with_params(
        &self,
        sheet_id: u64,
        include: impl Into<Option<Vec<SheetIncludeFlags>>>,
        exclude: impl Into<Option<Vec<SheetExcludeFlags>>>,
        row_ids: impl Into<Option<Vec<u64>>>,
        row_numbers: impl Into<Option<Vec<u64>>>,
        column_ids: impl Into<Option<Vec<u64>>>,
        rows_modified_since: impl Into<Option<&'a str>>, // TODO change this to a date type maybe
        level: impl Into<Option<Level>>,
    ) -> Result<Sheet> {
        let mut url = format!("{}/{}/{}", self.endpoint, "sheets", sheet_id);

        ParamBuilder::new(&mut url)
            .with_comma_separated_values("include", include.into())
            .with_comma_separated_values("exclude", exclude.into())
            .with_comma_separated_values("rowIds", row_ids.into())
            .with_comma_separated_values("rowNumbers", row_numbers.into())
            .with_comma_separated_values("columnIds", column_ids.into())
            .with_value("rowsModifiedSince", rows_modified_since.into())
            .with_value("level", level.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        // Note: I've timed the different methods for converting response data
        // to a `struct` type, and found the buffered reader approach to work
        // slightly better on average (at least on a Mac OS)

        // 1. Bytes
        #[cfg(feature = "serde-alloc")]
        let sheet = into_struct_from_slice(res).await?;

        // 2. String
        // let sheet = into_struct_from_str(res).await?;

        // 3. (Buffered) Reader
        #[cfg(not(feature = "serde-alloc"))]
        let sheet = resp_into_struct(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(sheet)
    }

    /// **Get Row** - Retrieves the specified row from a sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows from.
    /// * `row_id` - The specified row to retrieve.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-row
    ///
    pub async fn get_row(&self, sheet_id: u64, row_id: u64) -> Result<Row> {
        self.get_row_with_params(sheet_id, row_id, None, None, None)
            .await
    }

    /// **Get Row** - Retrieves the specified row from a sheet, with included _column data_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows from.
    /// * `row_id` - The specified row to retrieve.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-row
    ///
    pub async fn get_row_with_column_data(&self, sheet_id: u64, row_id: u64) -> Result<Row> {
        let include_flags = Some(vec![RowIncludeFlags::Columns]);
        self.get_row_with_params(sheet_id, row_id, include_flags, None, None)
            .await
    }

    /// **Get Row** - Retrieves the specified row from a sheet, with included
    /// _Multi-contact data_.
    ///
    /// # Note
    ///
    /// This is a convenience method to retrieve a Row with the `MULTI_CONTACT`
    /// cell data correctly populated. This is primarily important so that we can
    /// retrieve the email addresses for such cells, for example.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows from.
    /// * `row_id` - The specified row to retrieve.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-row
    ///
    pub async fn get_row_with_multi_contact_info(&self, sheet_id: u64, row_id: u64) -> Result<Row> {
        self.get_row_with_params(
            sheet_id,
            row_id,
            Some(vec![RowIncludeFlags::ObjectValue]),
            None,
            Some(Level::MultiContact),
        )
        .await
    }

    /// **Get Row** - Retrieves the specified row from a sheet, with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows from.
    /// * `row_id` - The specified row to retrieve.
    /// * `include` - A comma-separated list of elements to include in the response.
    /// * `exclude` - A comma-separated list of elements to _not_ include in the response.
    /// * `level` - Specifies whether multi-contact data is returned in a
    ///             backwards-compatible, text format, or as multi-contact data.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-row
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    pub async fn get_row_with_params(
        &self,
        sheet_id: u64,
        row_id: u64,
        include: impl Into<Option<Vec<RowIncludeFlags>>>,
        exclude: impl Into<Option<Vec<RowExcludeFlags>>>,
        level: impl Into<Option<Level>>,
    ) -> Result<Row> {
        let mut url: String = format!(
            "{}/{}/{}/{}/{}",
            self.endpoint, "sheets", sheet_id, "rows", row_id
        );

        ParamBuilder::new(&mut url)
            .with_comma_separated_values("include", include.into())
            .with_comma_separated_values("exclude", exclude.into())
            .with_value("level", level.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        // asynchronously aggregate the chunks of the body
        let row = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(row)
    }

    /// **Add Rows** - Inserts one or more rows into the sheet.
    ///
    /// If you want to insert the rows in any position but the default, use
    /// [location-specifier attributes].
    ///
    /// [location-specifier attributes]: https://smartsheet.redoc.ly/tag/rowsRelated#section/Specify-Row-Location
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to add the rows to.
    /// * `rows` - An array (list) of new Rows with the cell values to add.
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/rows-addToSheet>
    ///
    pub async fn add_rows(&self, sheet_id: u64, rows: impl Into<Vec<Row>>) -> Result<RowResult> {
        self.add_rows_with_params(sheet_id, rows, None, None).await
    }

    /// **Add Rows** - Inserts one or more rows into the sheet, with included
    /// _query parameters_.
    ///
    /// If you want to insert the rows in any position but the default, use
    /// [location-specifier attributes].
    ///
    /// [location-specifier attributes]: https://smartsheet.redoc.ly/tag/rowsRelated#section/Specify-Row-Location
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to add the rows to.
    /// * `rows` - An array (list) of new Rows with the cell values to add.
    /// * `allow_partial_success` - Default: `false`. When specified with a value
    ///               of `true`, enables partial success for this bulk operation.
    ///               See [Partial Success] for more information.
    /// * `override_validation` - Default: `false`. If set to a value of `true`,
    ///               allows a cell value outside of the validation limits. You
    ///               must also specify **strict** on a per-cell level with a
    ///               value of **false** to bypass value type checking.
    ///
    /// [Partial Success]: https://smartsheet.redoc.ly/#section/Work-at-Scale/Bulk-Operations
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/rows-addToSheet>
    ///
    pub async fn add_rows_with_params(
        &self,
        sheet_id: u64,
        rows: impl Into<Vec<Row>>,
        allow_partial_success: impl Into<Option<bool>>,
        override_validation: impl Into<Option<bool>>,
    ) -> Result<RowResult> {
        self.add_or_update_rows(
            Method::POST,
            sheet_id,
            rows,
            allow_partial_success.into(),
            override_validation.into(),
        )
        .await
    }

    /// **Update Rows** - Updates cell values in the specified rows,
    /// expands/collapses the specified rows, and/or modifies the position of
    /// specified rows (including indenting/outdenting). For detailed
    /// information about changing row positions, see
    /// [location-specifier attributes].
    ///
    /// [location-specifier attributes]: https://smartsheet.redoc.ly/tag/rowsRelated#section/Specify-Row-Location
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to update the rows in.
    /// * `rows` - An array (list) of Rows with the updated cell values.
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/update-rows>
    ///
    pub async fn update_rows(&self, sheet_id: u64, rows: impl Into<Vec<Row>>) -> Result<RowResult> {
        self.update_rows_with_params(sheet_id, rows, None, None)
            .await
    }

    /// **Update Rows** - Updates cell values in the specified rows,
    /// with included _query parameters_.
    ///
    /// Alternatively, expands/collapses the specified rows, and/or modifies
    /// the position of specified rows (including indenting/outdenting). For
    /// detailed information about changing row positions, see
    /// [location-specifier attributes].
    ///
    /// [location-specifier attributes]: https://smartsheet.redoc.ly/tag/rowsRelated#section/Specify-Row-Location
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to update the rows in.
    /// * `rows` - An array (list) of Rows with the updated cell values.
    /// * `allow_partial_success` - When specified with a value of `true`, enables
    ///               partial success for this bulk operation. See [Partial
    ///               Success] for more information.
    /// * `override_validation` - If set to a value of `true`, allows a cell value
    ///               outside of the validation limits. You must also specify **strict**
    ///               on a per-cell level with a value of **false** to bypass value
    ///               type checking.
    ///
    /// [Partial Success]: https://smartsheet.redoc.ly/#section/Work-at-Scale/Bulk-Operations
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/update-rows>
    ///
    pub async fn update_rows_with_params(
        &self,
        sheet_id: u64,
        rows: impl Into<Vec<Row>>,
        allow_partial_success: impl Into<Option<bool>>,
        override_validation: impl Into<Option<bool>>,
    ) -> Result<RowResult> {
        self.add_or_update_rows(
            Method::PUT,
            sheet_id,
            rows,
            allow_partial_success.into(),
            override_validation.into(),
        )
        .await
    }

    /// Internal method to *add* or *update* rows in a sheet.
    pub(crate) async fn add_or_update_rows(
        &self,
        method: hyper::Method,
        sheet_id: u64,
        rows: impl Into<Vec<Row>>,
        allow_partial_success: Option<bool>,
        override_validation: Option<bool>,
    ) -> Result<RowResult> {
        // The endpoint to ADD or UPDATE rows is the same.
        let mut url: String = format!("{}/{}/{}/{}", self.endpoint, "sheets", sheet_id, "rows");

        ParamBuilder::new(&mut url)
            .with_value("allowPartialSuccess", allow_partial_success)
            .with_value("overrideValidation", override_validation)
            .build();

        debug!("URL: {}", url);

        let data = serde_json::to_vec(&rows.into())?;

        let req = Request::builder()
            .method(method)
            .uri(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::from(data))?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        // asynchronously aggregate the chunks of the body
        let result = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(result)
    }

    /// **Delete Rows** - Deletes one or more specified rows from the sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to delete the rows from.
    /// * `row_ids` - An array (list) containing the IDs of the Rows to
    ///               delete from the smartsheet.
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/delete-rows>
    ///
    pub async fn delete_rows<const N: usize>(
        &self,
        sheet_id: u64,
        row_ids: impl Into<[u64; N]>,
    ) -> Result<RowResult<u64>> {
        self.delete_rows_with_params(sheet_id, row_ids, None).await
    }

    /// **Delete Rows** - Deletes one or more specified rows from the sheet,
    /// with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to delete the rows from.
    /// * `row_ids` - An array (list) containing the IDs of the Rows to
    ///               delete from the smartsheet.
    /// * `ignore_rows_not_found` -  Default: `false`. If set to `false` and any of
    ///              the specified Row IDs are not found, no rows are deleted,
    ///              and the "not found" error is returned.
    ///
    /// # Docs
    /// - <https://smartsheet.redoc.ly/#operation/delete-rows>
    ///
    pub async fn delete_rows_with_params<const N: usize>(
        &self,
        sheet_id: u64,
        row_ids: impl Into<[u64; N]>,
        ignore_rows_not_found: impl Into<Option<bool>>,
    ) -> Result<RowResult<u64>> {
        // The endpoint to ADD or UPDATE rows is the same.
        let mut url: String = format!("{}/{}/{}/{}", self.endpoint, "sheets", sheet_id, "rows");

        ParamBuilder::new(&mut url)
            .with_array("ids", row_ids.into())
            .with_value("ignoreRowsNotFound", ignore_rows_not_found.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::delete(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        // asynchronously aggregate the chunks of the body
        let result = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(result)
    }

    /// **List Columns** - Gets a list of all columns belonging to the specified sheet.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-columns
    ///
    pub async fn list_columns(&self, sheet_id: u64) -> Result<IndexResult<Column>> {
        self.list_columns_with_params(sheet_id, None, None, None)
            .await
    }

    /// **List Columns** - Gets a list of all columns belonging to the
    /// specified sheet, with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the columns from.
    /// * `level` - Specifies whether multi-contact data is returned in a
    ///             backwards-compatible, text format, or as multi-contact data.
    /// * `include` - A comma-separated list of elements to include in the response.
    /// * `include_all` - If true, include all results (i.e. do not paginate).
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-columns
    ///
    pub async fn list_columns_with_params(
        &self,
        sheet_id: u64,
        level: impl Into<Option<Level>>,
        include: impl Into<Option<Vec<ColumnIncludeFlags>>>,
        include_all: impl Into<Option<bool>>,
    ) -> Result<IndexResult<Column>> {
        let mut url = format!("{}/{}/{}/{}", self.endpoint, "sheets", sheet_id, "columns");

        ParamBuilder::new(&mut url)
            .with_value("level", level.into())
            .with_comma_separated_values("include", include.into())
            .with_value("includeAll", include_all.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        let columns = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(columns)
    }

    /// **Get Column** - Retrieves a column by *id* from the specified sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the column for.
    /// * `column_id` - The Column Id to retrieve the data for.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-column
    ///
    pub async fn get_column(&self, sheet_id: u64, column_id: u64) -> Result<Column> {
        self.get_column_with_params(sheet_id, column_id, None, None)
            .await
    }

    /// **Get Column** - Retrieves a column by *id* from the specified sheet,
    /// with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the column for.
    /// * `column_id` - The Column Id to retrieve the data for.
    /// * `level` - Specifies whether multi-contact data is returned in a
    ///             backwards-compatible, text format, or as multi-contact data.
    /// * `include` - A comma-separated list of elements to include in the response.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-column
    ///
    pub async fn get_column_with_params(
        &self,
        sheet_id: u64,
        column_id: u64,
        level: impl Into<Option<Level>>,
        include: impl Into<Option<Vec<ColumnIncludeFlags>>>,
    ) -> Result<Column> {
        let mut url = format!(
            "{}/{}/{}/{}/{}",
            self.endpoint, "sheets", sheet_id, "columns", column_id
        );

        ParamBuilder::new(&mut url)
            .with_value("level", level.into())
            .with_comma_separated_values("include", include.into())
            .build();

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        let column = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(column)
    }

    /// **List Attachments** - Gets a list of all attachments that are on the
    /// sheet, including sheet, row, and discussion-level attachments.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the attachments for.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-attachments
    ///
    pub async fn list_attachments(&self, sheet_id: u64) -> Result<IndexResult<AttachmentMeta>> {
        let url = format!(
            "{}/{}/{}/{}",
            self.endpoint, "sheets", sheet_id, "attachments"
        );

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        let attachments = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(attachments)
    }

    /// **Get Attachment** - Retrieves an attachment by *id* from the
    /// specified sheet.
    ///
    /// # Notes
    ///
    /// Fetches a temporary URL that allows you to download an attachment. The
    /// `urlExpiresInMillis` attribute tells you how long the URL is valid.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the attachments for.
    /// * `attachment_id` - The Attachment Id to retrieve the data for.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-attachment
    ///
    pub async fn get_attachment(&self, sheet_id: u64, attachment_id: u64) -> Result<Attachment> {
        let url = format!(
            "{}/{}/{}/{}/{}",
            self.endpoint, "sheets", sheet_id, "attachments", attachment_id
        );

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        let attachment = into_struct_from_slice(res).await?;

        debug!("Deserialize: {:?}", start.elapsed());

        Ok(attachment)
    }

    /// **Get Sheet By Name** - Convenience function to retrieve a specified
    /// sheet by name. Used for those times when you don't know the Sheet Id.
    ///
    /// This will internally call `list_sheets` and then filter the response
    /// data by the sheet name. It returns the first matching name.
    ///
    /// Returns the sheet, including rows, and optionally populated with
    /// discussion and attachment objects.
    ///
    /// # Arguments
    ///
    /// * `sheet_name` - The name of the Smartsheet to filter results by.
    ///
    #[deprecated(
        since = "0.2.0",
        note = "please cache the sheet id and use `get_sheet` instead"
    )]
    pub async fn get_sheet_by_name(&self, sheet_name: &'a str) -> Result<Sheet> {
        // Display a warning that the usage of this method is not recommended
        warn!(
            "{}",
            "Calling `get_sheet_by_name()` is not recommended; it's \
                preferable to cache the sheet ID and call \
                `get_sheet()` instead."
        );

        // Get a fresh list of sheets
        let result = self.list_sheets_with_params(None, Some(true), None).await?;

        // Find the sheet by the provided name
        return match result
            .data
            .into_iter()
            .find(|sheet| sheet.name == sheet_name)
        {
            Some(sheet) => Ok(sheet),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!("The provided sheet `{}` was not found", sheet_name),
            ))),
        };
    }

    /// **Get Column By Title** - Convenience function to retrieve a specified
    /// column by title (name). Used for those times when you don't know the
    /// Column Id.
    ///
    /// This will internally call `list_columns` and then filter the response
    /// data by the column title. It returns the first matching name.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the column from.
    /// * `column_title` - The name of the column to filter results by.
    ///
    ///
    #[deprecated(
        since = "0.2.0",
        note = "please cache the column id and use `get_column` instead"
    )]
    pub async fn get_column_by_title(
        &self,
        sheet_id: u64,
        column_title: &'a str,
    ) -> Result<Column> {
        // Display a warning that the usage of this method is not recommended
        warn!(
            "{}",
            "Calling `get_column_by_title()` is not recommended; it's \
                preferable to cache the column ID and call \
                `get_column()` instead."
        );

        // Get a fresh list of columns
        let result = self
            .list_columns_with_params(sheet_id, None, None, Some(true))
            .await?;

        // Find the column by the provided name
        return match result
            .data
            .into_iter()
            .find(|column| column.title == column_title)
        {
            Some(column) => Ok(column),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!("The provided column `{}` was not found", column_title),
            ))),
        };
    }
}
