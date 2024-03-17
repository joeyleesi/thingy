use std::fmt;

#[macro_export]
macro_rules! last {
    ($arg:expr) => { $arg };
    ($head:expr, $($rest:expr),+) => {
        last!($($rest),+)
    };
}

#[macro_export]
macro_rules! header {
    ($dst:expr, $($arg:tt)*) => {{
        use crate::last;
        write!($dst, "### <code>{}</code> \\{{#{}}}", format_args!($($arg)*), last!($($arg)*))
    }};
}

#[macro_export]
macro_rules! lua {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, "```lua\n{}\n```", format_args!($($arg)*))
    };
    ($($arg:tt)*) => {
        format!("```lua\n{}\n```", format_args!($($arg)*))
    };
}

pub struct Table {
    title: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(title: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Table {
        Table {
            title,
            headers,
            rows,
        }
    }

    fn column_widths(&self) -> Vec<usize> {
        let mut widths = vec![0; self.headers.len()];
        for (i, row) in self.headers.iter().enumerate() {
            widths[i] = std::cmp::max(widths[i], row.len());
        }
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = std::cmp::max(widths[i], cell.len());
            }
        }
        widths
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let widths = self.column_widths();
        write!(f, "**{}**", self.title)?;
        write!(f, "\n")?;
        for (i, header) in self.headers.iter().enumerate() {
            write!(f, "| {: <width$} ", header, width = widths[i])?;
        }
        write!(f, "|")?;
        write!(f, "\n")?;
        for i in 0..self.headers.len() {
            write!(f, "|{:-<width$}", "", width = widths[i] + 2)?;
        }
        write!(f, "|")?;
        for row in &self.rows {
            write!(f, "\n")?;
            for (i, cell) in row.iter().enumerate() {
                write!(f, "| {: <width$} ", cell, width = widths[i])?;
            }
            write!(f, "|")?;
        }
        Ok(())
    }
}
