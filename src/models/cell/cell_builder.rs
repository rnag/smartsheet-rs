use crate::helpers::{ColumnMapper, ColumnNameToId};
use crate::models::{Cell, CellValue, Contact, Hyperlink, ObjectType};
use crate::types::Result;

use serde_json::{json, to_value};
use std::io::{Error, ErrorKind};

/// **Cell Builder** - Utility to make it easier to construct a `Cell` object,
/// which is useful when adding or updating `Row`s in a `Sheet`.
///
pub struct CellBuilder<'a> {
    name_to_id: &'a ColumnNameToId<'a>,
}

impl<'a> CellBuilder<'a> {
    /// Create a new `CellBuilder` from a reference to a `ColumnMapper` object
    pub fn new(cols: &'a ColumnMapper) -> Self {
        Self {
            name_to_id: &cols.name_to_id,
        }
    }

    // TODO: Consider maybe re-rewriting this into a macro.
    pub fn new_cell<V: Into<CellValue>>(&'a self, column_name: &'a str, value: V) -> Result<Cell> {
        match self.name_to_id.get(column_name) {
            Some(&column_id) => Ok(self.new_cell_with_id(column_id, value)),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!(
                    "The column name `{}` does not exist in the sheet",
                    column_name
                ),
            ))),
        }
    }

    pub fn new_cell_with_id<V: Into<CellValue>>(&'a self, column_id: u64, value: V) -> Cell {
        Cell {
            column_id,
            value: Some(value.into()),
            ..Default::default()
        }
    }

    pub fn new_url_hyperlink_cell(
        &'a self,
        column_name: &'a str,
        display_text: &'a str,
        url: &'a str,
    ) -> Result<Cell> {
        match self.name_to_id.get(column_name) {
            Some(&column_id) => {
                Ok(self.new_url_hyperlink_cell_with_id(column_id, display_text, url))
            }
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!(
                    "The column name `{}` does not exist in the sheet",
                    column_name
                ),
            ))),
        }
    }

    pub fn new_url_hyperlink_cell_with_id(
        &'a self,
        column_id: u64,
        display_text: &'a str,
        url: &'a str,
    ) -> Cell {
        Cell {
            column_id,
            value: Some(display_text.into()),
            hyperlink: Some(Hyperlink::from(url)),
            ..Default::default()
        }
    }

    pub fn new_multi_picklist_cell(
        &'a self,
        column_name: &'a str,
        values: &[&'a str],
    ) -> Result<Cell> {
        match self.name_to_id.get(column_name) {
            Some(&column_id) => Ok(self.new_multi_picklist_cell_with_id(column_id, values)),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!(
                    "The column name `{}` does not exist in the sheet",
                    column_name
                ),
            ))),
        }
    }

    pub fn new_multi_picklist_cell_with_id(&'a self, column_id: u64, values: &[&'a str]) -> Cell {
        Cell {
            column_id,
            object_value: Some(json!(
            {
              "objectType": ObjectType::MultiPicklist.to_string(),
              "values": values
            })),
            ..Default::default()
        }
    }

    pub fn new_contact_cell(
        &'a self,
        column_name: &'a str,
        contact: impl Into<Contact<'a>>,
    ) -> Result<Cell> {
        match self.name_to_id.get(column_name) {
            Some(&column_id) => Ok(self.new_contact_cell_with_id(column_id, contact)),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!(
                    "The column name `{}` does not exist in the sheet",
                    column_name
                ),
            ))),
        }
    }

    pub fn new_contact_cell_with_id(
        &'a self,
        column_id: u64,
        contact: impl Into<Contact<'a>>,
    ) -> Cell {
        Cell {
            column_id,
            object_value: Some(to_value(contact.into()).unwrap()),
            ..Default::default()
        }
    }

    pub fn new_multi_contact_cell(
        &'a self,
        column_name: &'a str,
        contacts: &[Contact<'a>],
    ) -> Result<Cell> {
        match self.name_to_id.get(column_name) {
            Some(&column_id) => Ok(self.new_multi_contact_cell_with_id(column_id, contacts)),
            None => Err(Box::from(Error::new(
                ErrorKind::NotFound,
                format!(
                    "The column name `{}` does not exist in the sheet",
                    column_name
                ),
            ))),
        }
    }

    pub fn new_multi_contact_cell_with_id(
        &'a self,
        column_id: u64,
        contacts: &[Contact<'a>],
        // TODO I can't get this to WORK!! spent too long on it.
        // contacts: &[impl Into<Contact<'a>>],
    ) -> Cell {
        // let c: Vec<Contact<'a>> = contacts.into_iter().map(|c| c.into()).collect();
        let values = to_value(contacts).unwrap();

        Cell {
            column_id,
            object_value: Some(json!(
            {
                "objectType": ObjectType::MultiContact.to_string(),
                "values": values
            })),
            ..Default::default()
        }
    }
}
