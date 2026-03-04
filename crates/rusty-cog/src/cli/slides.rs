use clap::{Args, Subcommand};

/// Google Slides operations.
#[derive(Args, Debug)]
pub struct SlidesArgs {
    #[command(subcommand)]
    pub command: SlidesCommands,
}

#[derive(Subcommand, Debug)]
pub enum SlidesCommands {
    /// Show presentation metadata and info
    Info {
        /// Presentation ID
        id: String,
    },

    /// Create a new presentation
    Create {
        /// Presentation title
        #[arg(long)]
        title: String,

        /// Parent folder ID in Drive
        #[arg(long)]
        parent: Option<String>,
    },

    /// Create a presentation from Markdown
    CreateFromMarkdown {
        /// Markdown file path
        #[arg(long)]
        file: String,

        /// Presentation title
        #[arg(long)]
        title: Option<String>,

        /// Parent folder ID in Drive
        #[arg(long)]
        parent: Option<String>,
    },

    /// Copy an existing presentation
    Copy {
        /// Source presentation ID
        id: String,

        /// Title for the copy
        #[arg(long)]
        title: Option<String>,

        /// Destination folder ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// Export presentation to a file format
    Export {
        /// Presentation ID
        id: String,

        /// Export format (pdf, pptx, odp, txt)
        #[arg(long)]
        format: Option<String>,

        /// Output file path
        #[arg(long)]
        out: Option<String>,
    },

    /// List slides in a presentation
    ListSlides {
        /// Presentation ID
        id: String,
    },

    /// Add a new slide
    AddSlide {
        /// Presentation ID
        id: String,

        /// Predefined layout (BLANK, TITLE, TITLE_AND_BODY, etc.)
        #[arg(long)]
        layout: Option<String>,

        /// Insert at position (0-based index)
        #[arg(long)]
        index: Option<u32>,
    },

    /// Update speaker notes for a slide
    UpdateNotes {
        /// Presentation ID
        id: String,

        /// Slide object ID
        #[arg(long)]
        slide_id: String,

        /// Notes text
        #[arg(long)]
        text: String,
    },

    /// Replace all content in a slide with new content
    ReplaceSlide {
        /// Presentation ID
        id: String,

        /// Slide object ID
        #[arg(long)]
        slide_id: String,

        /// Find text
        #[arg(long)]
        find: String,

        /// Replace with text
        #[arg(long)]
        replace: String,

        /// Case-sensitive match
        #[arg(long)]
        match_case: bool,
    },
}
