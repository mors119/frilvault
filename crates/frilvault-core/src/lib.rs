pub mod app;
pub mod error;
pub mod note;

mod constants;
mod parser;
mod runtime;
mod storage;
mod workspace;

pub use app::*;
pub use error::*;
pub use note::*;
pub use parser::*;
pub use runtime::*;
pub use storage::*;
pub use workspace::*;

pub type FrilVaultResult<T> = Result<T, FrilVaultError>;

#[cfg(test)]
mod tests;
