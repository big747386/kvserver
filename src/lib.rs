mod kvstore;
mod errors;

pub use kvstore::KvStore;
pub use errors::{KvsError, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


