
pub mod network;
pub mod client;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        super::client::connect()
    }
}
