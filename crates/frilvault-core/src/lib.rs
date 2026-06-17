pub mod constants;
pub mod error;
pub mod note;
pub mod parser;
pub mod runtime;
pub mod storage;
pub mod workspace;

pub use error::*;
pub use note::*;
pub use parser::*;
pub use runtime::*;
pub use storage::*;
pub use workspace::*;

pub type FrilVaultResult<T> = Result<T, FrilVaultError>;

#[cfg(test)]
mod tests;
