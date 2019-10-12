use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal error occurred: {}", description)]
    InternalError {
        description: String,
    },

    #[fail(display = "path does not exist: {}", name)]
    PathDoesNotExist {
        name: String,
    },

    #[fail(display = "target given is an absolute: {}; only relative fragments are allowed", name)]
    TargetIsAbsolute {
        name: String,
    }
}