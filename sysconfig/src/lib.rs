#![no_std]
/// This crate contains hardware specific constants. It is meant to be replaced
/// by a project/application specific version (e.g. by overriding the
/// dependency in Cargo.toml or .cargo/config

///Frequency of the core clock in Hz
pub const SYS_CLOCK: u64 = 40_000_000;
