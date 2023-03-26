#[derive(Debug)]
pub enum Error {
    Env(String),
    Other(String),
    Parse {
        key: String,
        value: String,
        error: Option<String>,
    },
    Unspecified,
}

impl Error {
    fn description(&self) -> String {
        match self {
            Error::Env(s) => format!("Missing environment variable: {}", s),
            Error::Parse { key, value, error } => {
                let mut s = format!("Failed to parse {} as {}", value, key);
                if let Some(e) = error {
                    s.push_str(&format!(": {}", e));
                }
                s
            }
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
        Error::Env(e.to_string())
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
