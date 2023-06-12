mod constants;
mod consumer;
mod core;
mod helper;
mod memphis_client;
mod models;
mod options;
mod producer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_key() {
        let key = helper::memphis_util::get_unique_key(10);
        assert_eq!(key.len(), 10);
    }
}
