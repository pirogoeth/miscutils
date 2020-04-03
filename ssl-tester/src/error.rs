use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal error occurred: {}", description)]
    InternalError { description: String },

    #[fail(display = "unknown SSL/TLS protocol version: {}", version)]
    UnknownProtocolVersion { version: String },

    #[fail(display = "unknown SSL verify mode: {}", mode)]
    UnknownSslVerifyMode { mode: String },

    #[fail(display = "conflicting options: {} and {}", 0, 1)]
    ConflictingOptions (&'static str, &'static str),
}

