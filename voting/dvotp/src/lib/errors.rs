use std::option;
use failure::Error;
use serde_cbor;

use swth_cli_libv2::errors::CliError;

impl From<option::NoneError> for LibError {
    fn from(_err: option::NoneError) -> Self {
        LibError::NoneError
    }
}

impl From<serde_cbor::error::Error> for LibError {
    fn from(_err: serde_cbor::error::Error) -> Self {
        LibError::SerdeCborError
    }
}

impl From<CliError> for LibError {
    fn from(_err: CliError) -> Self {
        LibError::CliError
    }
}

#[derive(Debug, Fail)]
pub enum LibError {
    #[fail(display = "{} must be {} characters in length. Got {} characters.\n", field, size, got)]
    LengthError {
        field: String,
        size: usize,
        got: usize
    },
    #[fail(display = "{} can only contain {} characters. Got {:?}\n", field, encoding, got)]
    EncodingError {
        field: String,
        encoding: String,
        got: String
    },
    #[fail(display = "field {} cannot be empty or zero.\n", field)]
    EmptyZeroError {
        field: String,
    },
    #[fail(display = "{}\n", contents)]
    CustomError {
        contents: String
    },
    #[fail(display = "User {} already exists in our user database. Cannot create duplicate user.", user)]
    UserExistsError {
        user: String
    },
    #[fail(display = "{} does not exist in {}", contents, structure)]
    NExistKeyError {
        contents: String,
        structure: String
    },
    #[fail(display = "Experienced integer overlow at: {}. Offending integers were {} and {} for integer size {}", origin, fst, snd, intsize)]
    IntOverflowError {
        origin: String,
        fst: usize,
        snd: usize,
        intsize: String,
    },
    #[fail(display = "Experienced integer underflow at: {}. Offending values were {} and {}, for type {}", origin, fst, snd, intsize)]
    IntUnderflowError {
        origin: String,
        fst: usize,
        snd: usize,
        intsize: String
    },
    #[fail(display = "Serde CBOR failed to decode the HTTP response from CBOR")]
    SerdeCborError,
    #[fail(display = "Got a 'None' when unwrapping a result")]
    NoneError,

    #[fail(display = "Received a CLI type error (the one from swth_cli_libv2); the only places that's used here are in the to_swth_transaction and to_cli_req methods for the wrapper type")]
    CliError,


}
