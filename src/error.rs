#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    Other(String),
    FailedToParse {
        key: String,
        value: String,
        error: Option<String>,
    },
    MissingConfig(String),
    InvalidConfig(String),
    Database(sqlx::Error),
    Unspecified,
}

impl Error {
    fn description(&self) -> String {
        match self {
            Error::FailedToParse { key, value, error } => {
                let mut s = format!("Failed to parse {} as {}", value, key);
                if let Some(e) = error {
                    s.push_str(&format!(": {}", e));
                }
                s
            }
            Error::MissingEnv(s) => format!("Missing environment variable: {}", s),
            Error::MissingConfig(s) => format!("Missing config value: {}", s),
            Error::InvalidConfig(s) => format!("Invalid config value: {}", s),
            Error::Database(e) => e.to_string(),
            Error::Other(s) => s.to_string(),
            Error::Unspecified => "Unspecified error, please report this".to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for Error {}

impl From<&dyn std::error::Error> for Error {
    fn from(e: &dyn std::error::Error) -> Self {
        Error::Other(get_stacktrace(e))
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::MissingEnv(e.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e)
    }
}

fn get_stacktrace(e: &dyn std::error::Error) -> String {
    let mut s = vec![];
    let mut source = Some(e);
    while let Some(e) = source {
        s.push(e.to_string());
        source = e.source();
    }
    s.join("\n")
}
