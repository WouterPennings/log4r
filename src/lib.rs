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
/// use logr;
/// use logr::LogTypes;
/// let s: String = String::from("This is a custom log");
/// logr::log(s, LogTypes::Custom("\x1b[1;97m\x1b[1;101m".to_string()));
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
/// use logr;
/// let s: String = String::from("User has logged in");
/// logr::success(s);
/// ```
pub fn success(s: String) {
    log(s, LogTypes::Success)
}

/// Prints a blue message in the terminal with a `[INFO]` label
///
/// ```
/// use logr;
/// let s: String = String::from("User has logged in");
/// logr::info(s);
/// ```
pub fn info(s: String) {
    log(s, LogTypes::Info)
}

/// Prints a yellow message in the terminal with a `[WARNING]` label
///
/// ```
/// use logr;
/// let s: String = String::from("User has logged in");
/// logr::warning(s);
/// ```
pub fn warning(s: String) {
    log(s, LogTypes::Warning)
}

/// Prints a red message in the terminal with a `[ERROR]` label
///
/// ```
/// use logr;
/// let s: String = String::from("User has logged in");
/// logr::error(s);
/// ```
pub fn error(s: String) {
    log(s, LogTypes::Error)
}

/// Prints a white and red message in the terminal with a `[CRITICAL]` label
///
/// ```
/// use logr;
/// let s: String = String::from("User has logged in");
/// logr::critical(s);
/// ```
pub fn critical(s: String) {
    log(s, LogTypes::Critical)
}
