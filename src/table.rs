// Copyright (c) 2015 Sergey "SnakE" Gromov
//
// See the file license.txt for copying permission.

//! # Aligned Table

use std::cmp;

/// A table of strings that can print its columns vertically aligned.
pub struct Table {
    t: Vec<Row>,
}

impl Table {
    /// Create a new, empty table.
    pub fn new() -> Table {
        Table { t: Vec::new() }
    }

    /// Add a row to the table.
    pub fn push(&mut self, row: Row) {
        self.t.push(row);
    }

    fn measure(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = Vec::new();

        for row in &self.t {
            for (i, cell) in row.cells.iter().enumerate() {
                let w = cell.chars().count();
                if widths.len() == i {
                    widths.push(w);
                } else {
                    widths[i] = cmp::max(widths[i], w);
                }
            }
        }

        widths
    }

    /// Print aligned rows to stdout.
    pub fn print(&self) {
        let widths = self.measure();

        for row in &self.t {
            for (i, cell) in row.cells.iter().enumerate() {
                let w = widths[i];
                print!("  {1:>0$.0$}", w, cell);
            }
            if let Some(ref s) = row.span {
                print!("  {}", s);
            }
            println!("");
        }
    }
}

/// One row in a table.
///
/// A row may contain an arbitrary number of strings, each occupying one cell.
/// Strings are right-aligned within their cells when printed.
///
/// The last item can optionally span the rest of the cells in the row. No
/// items can be pushed after it, and such a string is left-aligned when printed.
pub struct Row {
    cells: Vec<String>,     // values for each column
    span: Option<String>,   // optional last item that spans the rest of the columns
}

impl Row {
    /// Create a new, empty row.
    pub fn new() -> Row {
        Row { cells: Vec::new(), span: None }
    }

    /// Push a string that occupies one column and aligns to the right.
    ///
    /// This method panics if `push_span()` was called on the row.
    pub fn push(&mut self, s: &str) {
        if self.span.is_some() {
            panic!("all columns are already spanned");
        }
        self.cells.push(s.to_string());
    }

    /// Push a string that spans the rest of the columns and aligns to the left.
    ///
    /// Nothing else can be pushed on the row after `push_span()` is called.
    pub fn push_span(&mut self, s: &str) {
        self.span = Some(s.to_string());
    }
}
