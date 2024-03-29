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
❯❯ export SMARTSHEET_ACCESS_TOKEN='MY-TOKEN'
```

*On Windows*, that would instead take the following form:

```shell
❯❯ $env:SMARTSHEET_ACCESS_TOKEN = 'MY-TOKEN'
```

Once that is done, you should be able to use
any of the examples to make sample calls to the Smartsheet
API. 

## Quickstart

[cargo-rx]: https://github.com/rnag/cargo-rx

Install my crate [cargo-rx], which abstracts away `cargo run --example`.
This provides a single `rx` command.

```shell
❯❯ cargo install cargo-rx
```

Now start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/smartsheet-rs.git
```

Then, simply `cd` into the project folder:

```shell
❯❯ cd smartsheet-rs
```

From here, you can use `rx` to build and run
any of the examples individually.

If you run the command without any arguments, you can select
from the list of available examples:

```shell
❯❯ rx
```

In particular, here's a simple example
of retrieving a list of all sheets in the account:

```shell
❯❯ rx sheets
```

To get the ID of a sheet, you can either examine the
output from the above result, or you can find it under 
the Sheet *Settings* page in the Smartsheet web UI.

Once you have the *sheet ID*, you can - for example -
retrieve a list of all columns in the sheet:

> Note: to pass arguments to a script, you can include them after the `--`.

```shell
❯❯ rx columns -- <sheet-id>
```
