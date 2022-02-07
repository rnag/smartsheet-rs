# Examples

This folder contains example scripts that can be used
to make calls to the [Smartsheet API](https://smartsheet-platform.github.io/api-docs/).

To start out with, you'll need to create an *access token*
as mentioned in the [Getting Started](https://smartsheet-platform.github.io/api-docs/#getting-started)
guide.

After that, you will need to ensure that the **SMARTSHEET_ACCESS_TOKEN**
env variable is properly set.

*On Mac/Linux*, this would be like:

```shell
❯❯ export SMARTSHEET_ACCESS_TOKEN = 'MY-TOKEN'
```

*On Windows*, that would instead take the following form:

```shell
❯❯ $env:SMARTSHEET_ACCESS_TOKEN = 'MY-TOKEN'
```

Once that is done, you should be able to use
any of the examples to make sample calls to the Smartsheet
API. 

## Quickstart

First, start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/smartsheet-rs.git
```

When running the examples, we'll often want to see the DEBUG logs from the library under test,
`smartsheet-rs` in this case. Therefore, remember to ensure that the **RUST_LOG** env variable
is properly set.

For example, on *Mac/Linux*:

```shell
❯❯ export RUST_LOG='smartsheet_rs=TRACE'
```

On *Windows*:

```shell
❯❯ $env:RUST_LOG='smartsheet_rs=TRACE'
```

Next, simply just `cd` into the project folder:

```shell
❯❯ cd smartsheet-rs
```

From here, you can use `cargo` to build and run
any of the examples individually.

In particular, here's a simple example
of retrieving a list of all sheets in the account:

```shell
❯❯ cargo run --example sheets
```

To get the ID of a sheet, you can either examine the
output from the above result, or you can find it under 
the Sheet *Settings* page in the Smartsheet web UI.

Once you have the *sheet ID*, you can - for example -
retrieve a list of all columns in the sheet:

```shell
❯❯ cargo run --example columns <sheet-id>
```
