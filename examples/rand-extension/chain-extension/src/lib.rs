#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(feature = "ink")]
pub mod ink;

#[obce::error]
pub enum RandomReadErr {
    FailGetRandomSource,
}

#[obce::definition(id = "rand-extension@v0.1")]
pub trait RandExtension {
    fn fetch_random(&self, subject: [u8; 32]) -> Result<[u8; 32], RandomReadErr>;
}
