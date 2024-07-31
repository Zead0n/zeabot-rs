pub mod discord;

// TODO: Remove this when all calls are removed
pub fn check_result<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(success) => return success,
        Err(e) => panic!("Error in result: {:?}", e),
    }
}
