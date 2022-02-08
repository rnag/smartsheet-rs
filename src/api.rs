///! Smartsheet API v2 implementation in Rust
///!
use crate::auth::auth_token;
use crate::builders::ParamBuilder;
use crate::constants::{API_ENDPOINT, ENV_VAR_NAME};
use crate::models::*;
use crate::status::raise_for_status;
use crate::types::Result;
use crate::utils::{into_struct_from_slice, resp_into_struct};

use std::io::{Error, ErrorKind};
use std::time::Instant;

use hyper::client::HttpConnector;
use hyper::header::AUTHORIZATION;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use log::debug;

/// Client implementation for making requests to the *Smartsheet
/// API v2*
///
/// # Links
/// - [`smartsheet-rs`](https://docs.rs/smartsheet-rs)
/// - [Official Documentation](https://smartsheet-platform.github.io/api-docs/)
///
pub struct SmartsheetApi<'a> {
    bearer_token: String,
    client: Client<HttpsConnector<HttpConnector>>,
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

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

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
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-sheets
    ///
    pub async fn list_sheets_with_params(
        &self,
        include: Option<Vec<ListSheetIncludeFlags>>,
        include_all: Option<bool>,
        modified_since: Option<&str>, // TODO change this to a date type mayb
    ) -> Result<IndexResult<Sheet>> {
        let mut url = format!("{}/{}", self.endpoint, "sheets");

        let mut params = ParamBuilder::new();

        params.insert_comma_separated_values("include", include);
        params.insert_value("includeAll", include_all);
        params.insert_value("modifiedSince", modified_since);

        params.add_query_to_url(&mut url);

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
        self.get_sheet_with_params(sheet_id, None, None, None, None, None, None)
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
    /// * `rows_modified_since` - Date should be in ISO-8601 format, for
    ///                           example, `2020-01-30T13:25:32-07:00`
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-sheet
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    pub async fn get_sheet_with_params(
        &self,
        sheet_id: u64,
        include: Option<Vec<SheetIncludeFlags>>,
        exclude: Option<Vec<SheetExcludeFlags>>,
        row_ids: Option<Vec<u64>>,
        row_numbers: Option<Vec<u64>>,
        column_ids: Option<Vec<u64>>,
        rows_modified_since: Option<&str>,
    ) -> Result<Sheet> {
        let mut url = format!("{}/{}/{}", self.endpoint, "sheets", sheet_id);

        let mut params = ParamBuilder::new();

        params.insert_comma_separated_values("include", include);
        params.insert_comma_separated_values("exclude", exclude);
        params.insert_comma_separated_values("rowIds", row_ids);
        params.insert_comma_separated_values("rowNumbers", row_numbers);
        params.insert_comma_separated_values("columnIds", column_ids);
        params.insert_value("rowsModifiedSince", rows_modified_since);

        params.add_query_to_url(&mut url);

        debug!("URL: {}", url);

        let req = Request::get(&url)
            .header(AUTHORIZATION, &self.bearer_token)
            .body(Body::empty())?;

        let mut res = self.client.request(req).await?;
        raise_for_status(url, &mut res).await?;

        let start = Instant::now();

        // Note: I've timed the different methods for converting response data
        // to a `struct` type, and found the buffered reader approach to work
        // out the best for this approach. The response time seems to be quite
        // stable where the reader implementation is used.

        // 1. Bytes
        // let sheet = into_struct_from_slice(res).await?;

        // 2. String
        // let sheet = into_struct_from_str(res).await?;

        // 3. (Buffered) Reader
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

    /// **Get Row** - Retrieves the specified row from a sheet, with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the rows from.
    /// * `row_id` - The specified row to retrieve.
    /// * `include` - A comma-separated list of elements to include in the response.
    /// * `exclude` - A comma-separated list of elements to _not_ include in the response.
    /// * `level` - Specifies whether multi-contact data is returned in a backwards-compatible,
    /// text format, or as multi-contact data.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#get-row
    /// - https://smartsheet-platform.github.io/api-docs/#row-include-flags
    ///
    pub async fn get_row_with_params(
        &self,
        sheet_id: u64,
        row_id: u64,
        include: Option<Vec<RowIncludeFlags>>,
        exclude: Option<Vec<RowExcludeFlags>>,
        level: Option<Level>,
    ) -> Result<Row> {
        let mut url: String = format!(
            "{}/{}/{}/{}/{}",
            self.endpoint, "sheets", sheet_id, "rows", row_id
        );

        let mut params = ParamBuilder::new();

        params.insert_comma_separated_values("include", include);
        params.insert_comma_separated_values("exclude", exclude);
        params.insert_value("level", level);

        params.add_query_to_url(&mut url);

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

    /// **List Columns** - Gets a list of all columns belonging to the specified sheet.
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-columns
    ///
    pub async fn list_columns(&self, sheet_id: u64) -> Result<IndexResult<Column>> {
        self.list_columns_with_params(sheet_id, None, None).await
    }

    /// **List Columns** - Gets a list of all columns belonging to the
    /// specified sheet, with included _query parameters_.
    ///
    /// # Arguments
    ///
    /// * `sheet_id` - The Smartsheet to retrieve the columns from.
    /// * `level` - Specifies whether multi-contact data is returned in a backwards-compatible,
    /// text format, or as multi-contact data.
    /// * `include_all` - If true, include all results (i.e. do not paginate).
    ///
    /// # Docs
    /// - https://smartsheet-platform.github.io/api-docs/#list-columns
    ///
    pub async fn list_columns_with_params(
        &self,
        sheet_id: u64,
        level: Option<Level>,
        include_all: Option<bool>,
    ) -> Result<IndexResult<Column>> {
        let mut url = format!("{}/{}/{}/{}", self.endpoint, "sheets", sheet_id, "columns");

        let mut params = ParamBuilder::new();
        params.insert_value("level", level);
        params.insert_value("includeAll", include_all);
        params.add_query_to_url(&mut url);

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
}
