pub mod command;
pub mod toml_struct;
pub(crate) mod traits;

pub use command::{Cli, New, Add, Fix, Rename, Delete};


#[cfg(test)]
mod tests{
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn new_should_work(){

    }
}