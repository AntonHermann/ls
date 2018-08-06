use std::io;
use std::time;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SystemTimeError(time::SystemTimeError),
    ParseIdError, // users::User or users::Group couldn't be matched
}
impl From<io::Error> for Error { fn from(e: io::Error) -> Self { Error::IOError(e) } }
impl From<time::SystemTimeError> for Error { fn from(e: time::SystemTimeError) -> Self { Error::SystemTimeError(e) } }