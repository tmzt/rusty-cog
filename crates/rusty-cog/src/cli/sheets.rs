use clap::{Args, Subcommand};

/// Google Sheets operations.
#[derive(Args, Debug)]
pub struct SheetsArgs {
    #[command(subcommand)]
    pub command: SheetsCommands,
}

#[derive(Subcommand, Debug)]
pub enum SheetsCommands {
    /// Show spreadsheet metadata
    Metadata {
        /// Spreadsheet ID
        id: String,
    },

    /// Get cell values from a range
    Get {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range (e.g. "Sheet1!A1:C10")
        #[arg(long)]
        range: String,

        /// Value render option (FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA)
        #[arg(long)]
        value_render: Option<String>,
    },

    /// Create a new spreadsheet
    Create {
        /// Spreadsheet title
        #[arg(long)]
        title: String,

        /// Sheet names to create (repeatable)
        #[arg(long, num_args = 1..)]
        sheets: Vec<String>,

        /// Parent folder ID in Drive
        #[arg(long)]
        parent: Option<String>,
    },

    /// Update cell values in a range
    Update {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range
        #[arg(long)]
        range: String,

        /// Values as JSON array of arrays
        #[arg(long)]
        values: Option<String>,

        /// Read values from file
        #[arg(long)]
        file: Option<String>,

        /// Value input option (RAW, USER_ENTERED)
        #[arg(long)]
        value_input: Option<String>,
    },

    /// Append rows to a sheet
    Append {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range (target sheet)
        #[arg(long)]
        range: String,

        /// Values as JSON array of arrays
        #[arg(long)]
        values: Option<String>,

        /// Read values from file
        #[arg(long)]
        file: Option<String>,

        /// Value input option (RAW, USER_ENTERED)
        #[arg(long)]
        value_input: Option<String>,
    },

    /// Clear cell values in a range
    Clear {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range to clear
        #[arg(long)]
        range: String,
    },

    /// Apply formatting to cells
    Format {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range
        #[arg(long)]
        range: String,

        /// Bold text
        #[arg(long)]
        bold: bool,

        /// Italic text
        #[arg(long)]
        italic: bool,

        /// Font size
        #[arg(long)]
        font_size: Option<u32>,

        /// Background color (hex)
        #[arg(long)]
        background_color: Option<String>,

        /// Text color (hex)
        #[arg(long)]
        text_color: Option<String>,

        /// Number format pattern
        #[arg(long)]
        number_format: Option<String>,
    },

    /// Insert rows or columns
    Insert {
        /// Spreadsheet ID
        id: String,

        /// Sheet ID (numeric)
        #[arg(long)]
        sheet_id: u32,

        /// Dimension (ROWS, COLUMNS)
        #[arg(long)]
        dimension: String,

        /// Start index (0-based)
        #[arg(long)]
        start: u32,

        /// End index (exclusive, 0-based)
        #[arg(long)]
        end: u32,
    },

    /// Manage cell notes
    Notes {
        /// Spreadsheet ID
        id: String,

        /// A1 notation range
        #[arg(long)]
        range: String,

        /// Note text (omit to read existing notes)
        #[arg(long)]
        text: Option<String>,

        /// Clear notes in range
        #[arg(long)]
        clear: bool,
    },

    /// Export spreadsheet to a file format
    Export {
        /// Spreadsheet ID
        id: String,

        /// Export format (xlsx, pdf, csv, tsv, ods, html)
        #[arg(long)]
        format: Option<String>,

        /// Output file path
        #[arg(long)]
        out: Option<String>,

        /// Sheet name/ID to export (for single-sheet formats like CSV)
        #[arg(long)]
        sheet: Option<String>,
    },

    /// Copy a sheet to another spreadsheet
    Copy {
        /// Source spreadsheet ID
        id: String,

        /// Source sheet ID (numeric)
        #[arg(long)]
        sheet_id: u32,

        /// Destination spreadsheet ID
        #[arg(long)]
        destination: String,
    },
}
