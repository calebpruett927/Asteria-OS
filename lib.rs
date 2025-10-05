// ui crate
#![cfg_attr(not(feature = "devhost"), no_std)]
#[cfg(feature = "devhost")]
pub fn stub() { /* devhost ok */ }
