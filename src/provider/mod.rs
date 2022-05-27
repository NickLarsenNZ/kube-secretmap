pub mod aws;

pub trait Provider {
    type Config;
    fn new(cfg: Self::Config) -> Self;
    fn retrieve_secret(&self) -> String;
}
