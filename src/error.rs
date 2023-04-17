pub type Result<T> = core::result::Result<T, Error>;

// for this we keep all errors here
//later it might be good practice to have a layers of errors
#[derive(Debug)]
pub enum Error {
    LoginFail,
}

// region : ErrorHandling
//we learned this on berline.rs from jörn
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion : ErrorHandling
