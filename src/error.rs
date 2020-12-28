use darling::Error as DarlingError;
use proc_macro::TokenStream;
use syn::Error as SynError;

pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for wrapping syn & darling errors as well as any other custom errors that may become necessary.
pub enum Error {
    Syn(SynError),
    Darling(DarlingError),
}

impl Error {
    pub fn to_compile_error(self) -> TokenStream {
        match self {
            Error::Syn(err) => err.to_compile_error().into(),
            Error::Darling(err) => err.write_errors().into(),
        }
    }
}

impl From<DarlingError> for Error {
    fn from(err: DarlingError) -> Self {
        Error::Darling(err)
    }
}

impl From<SynError> for Error {
    fn from(err: SynError) -> Self {
        Error::Syn(err)
    }
}
