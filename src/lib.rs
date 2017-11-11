extern crate jni_sys;

/// This crate allows to call Java code from Rust via JNI.

#[macro_use]
mod macros_jvm_objects;

mod jvm;
mod jvm_attachment;
mod jvm_class;
mod jvm_method;
mod jvm_object;
mod jvm_string;

pub use jni_sys::{
    jboolean, jbyte, jchar, jint, jdouble, jfloat, jlong, jobject, jshort, jvalue,
    JNI_FALSE, JNI_TRUE, JNIEnv,
};

pub use jvm::{
    jvalue_from_jboolean,
    jvalue_from_jbyte,
    jvalue_from_jchar,
    jvalue_from_jdouble,
    jvalue_from_jint,
    jvalue_from_jfloat,
    jvalue_from_jlong,
    jvalue_from_jobject,
    jvalue_from_jshort,
    Jvm
};
pub use jvm_class::JvmClass;
pub use jvm_method::JvmMethod;
pub use jvm_object::JvmObject;
pub use jvm_string::JvmString;
