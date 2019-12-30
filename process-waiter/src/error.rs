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
}