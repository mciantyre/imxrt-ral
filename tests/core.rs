//! Ensures that the core imxrt-ral features are always avaliable,
//! no matter the build.
//!
//! Failure manifests at compile time.

#![allow(unused)]
use imxrt_ral::{modify_reg, read_reg, write_reg, Access, Instance, Valid};
