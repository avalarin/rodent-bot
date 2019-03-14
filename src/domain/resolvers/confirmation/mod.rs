mod state;
mod resolver;

pub use self::state::*;
pub use self::resolver::*;

#[derive(Debug, Fail)]
pub enum ConfirmationResolverError {
    #[fail(display = "user is not present in context")]
    UserIsNotPresent {}
}
