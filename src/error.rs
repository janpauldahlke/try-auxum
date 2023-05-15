use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

// for this we keep all errors here
//later it might be good practice to have a layers of errors
#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,

    //ModelErrors TODO: refactor this to be in the model layer
    TicketDeleteFailIdNotFound { id: u64 },

    //AuthErrors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

//this is key to make the error handling work in Axum
// intoResponse is very important for all work in axum
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        //this will return Axum Response!
        println!("--> {:<12} - error - {self:?}", "INTORESPONSE");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "--> unhandled client error",
        )
            .into_response()
    }
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
