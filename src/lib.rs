use std::fmt::{Display, Formatter, Result};

pub enum LogTypes {
    Success,
    Info,
    Warning,
    Error,
    Critical,
    Custom(String),
}

impl Display for LogTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LogTypes::Success => write!(f, "\x1b[0;92m"),
            LogTypes::Info => write!(f, "\x1b[0;36m"),
            LogTypes::Warning => write!(f, "\x1b[0;93m"),
            LogTypes::Error => write!(f, "\x1b[0;91m"),
            LogTypes::Critical => write!(f, "\x1b[1;97m\x1b[1;101m"),
            LogTypes::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Prints a green message in the terminal with a `[SUCCESS]` label
///
/// ```
/// use log4r;
/// use log4r::LogTypes;
/// let s: String = String::from("This is a custom log");
/// log4r::log(s, LogTypes::Custom("\x1b[1;97m\x1b[1;101m".to_string()));
/// ```
pub fn log(str: String, log_type: LogTypes) {
    match log_type {
        LogTypes::Success => println!("{}[SUCCESS] {}\x1b[0;0m", log_type, str),
        LogTypes::Info => println!("{}[INFO] {}\x1b[0;0m", log_type, str),
        LogTypes::Warning => println!("{}[WARNING] {}\x1b[0;0m", log_type, str),
        LogTypes::Error => println!("{}[ERROR] {}\x1b[0;0m", log_type, str),
        LogTypes::Critical => println!("{}[URGENT] {}\x1b[0;0m", log_type, str),
        LogTypes::Custom(esc) =>  println!("{}[CUSTOM LOG]  {}\x1b[0;0m", esc, str),
    }
}

/// Prints a green message in the terminal with a `[SUCCESS]` label
///
/// ```
/// use log4r;
/// let s: String = String::from("User has logged in");
/// log4r::success(s);
/// ```
pub fn success(s: String) {
    log(s, LogTypes::Success)
}

/// Prints a blue message in the terminal with a `[INFO]` label
///
/// ```
/// use log4r;
/// let s: String = String::from("User has logged in");
/// log4r::info(s);
/// ```
pub fn info(s: String) {
    log(s, LogTypes::Info)
}

/// Prints a yellow message in the terminal with a `[WARNING]` label
///
/// ```
/// use log4r;
/// let s: String = String::from("User has logged in");
/// log4r::warning(s);
/// ```
pub fn warning(s: String) {
    log(s, LogTypes::Warning)
}

/// Prints a red message in the terminal with a `[ERROR]` label
///
/// ```
/// use log4r;
/// let s: String = String::from("User has logged in");
/// log4r::error(s);
/// ```
pub fn error(s: String) {
    log(s, LogTypes::Error)
}

/// Prints a white and red message in the terminal with a `[CRITICAL]` label
///
/// ```
/// use log4r;
/// let s: String = String::from("User has logged in");
/// log4r::critical(s);
/// ```
pub fn critical(s: String) {
    log(s, LogTypes::Critical)
}
