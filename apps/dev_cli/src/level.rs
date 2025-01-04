use clap::ValueEnum;
use tool_tracing::level::VerboseLevel;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VerboseSwapLevel {
    ERROR = 4,
    WARN = 3,
    INFO = 2,
    DEBUG = 1,
    TRACE = 0,
}

impl From<VerboseSwapLevel> for VerboseLevel {
    fn from(level: VerboseSwapLevel) -> Self {
        match level {
            VerboseSwapLevel::ERROR => VerboseLevel::ERROR,
            VerboseSwapLevel::WARN => VerboseLevel::WARN,
            VerboseSwapLevel::INFO => VerboseLevel::INFO,
            VerboseSwapLevel::DEBUG => VerboseLevel::DEBUG,
            VerboseSwapLevel::TRACE => VerboseLevel::TRACE,
        }
    }
}
