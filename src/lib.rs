extern crate jni_sys;

/// This crate allows to call Java code from Rust via JNI.

pub mod jvm;
pub mod jvm_class;
pub mod jvm_method;
pub mod jvm_object;