/// CLI exit codes matching gogcli.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    Ok = 0,
    Error = 1,
    Usage = 2,
    Empty = 3,
    AuthRequired = 4,
    NotFound = 5,
    PermissionDenied = 6,
    RateLimited = 7,
    Retryable = 8,
    Config = 10,
    Cancelled = 130,
}

impl ExitCode {
    pub fn code(self) -> i32 {
        self as i32
    }
}

impl From<&cog_core::Error> for ExitCode {
    fn from(e: &cog_core::Error) -> Self {
        match e.exit_code() {
            0 => Self::Ok,
            2 => Self::Usage,
            3 => Self::Empty,
            4 => Self::AuthRequired,
            5 => Self::NotFound,
            6 => Self::PermissionDenied,
            7 => Self::RateLimited,
            8 => Self::Retryable,
            10 => Self::Config,
            130 => Self::Cancelled,
            _ => Self::Error,
        }
    }
}
