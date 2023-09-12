use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    PermissionDenied = 403,
    NotFound = 404,
}

impl StatusCode {
    /** Returns the numerical status code as u16 */
    pub fn code(&self) -> u16 {
        *self as u16
    }
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let reason = match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::PermissionDenied => "Permission Denied"
        };
        f.write_str(reason)
    }
}