extern crate jni_sys;

/// This crate allows to call Java code from Rust via JNI.

#[macro_use]
mod macros_jvm_wrappers;

mod jvm;
mod jvm_array;
mod jvm_attachment;
mod jvm_boolean_array;
mod jvm_byte_array;
mod jvm_char_array;
mod jvm_class;
mod jvm_double_array;
mod jvm_float_array;
mod jvm_int_array;
mod jvm_long_array;
mod jvm_method;
mod jvm_object;
mod jvm_object_array;
mod jvm_short_array;
mod jvm_string;
mod jvm_throwable;

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
pub use jvm_array::JvmArray;
pub use jvm_boolean_array::JvmBooleanArray;
pub use jvm_byte_array::JvmByteArray;
pub use jvm_char_array::JvmCharArray;
pub use jvm_class::JvmClass;
pub use jvm_double_array::JvmDoubleArray;
pub use jvm_float_array::JvmFloatArray;
pub use jvm_int_array::JvmIntArray;
pub use jvm_long_array::JvmLongArray;
pub use jvm_method::JvmMethod;
pub use jvm_object::JvmObject;
pub use jvm_object_array::JvmObjectArray;
pub use jvm_string::JvmString;
pub use jvm_short_array::JvmShortArray;
pub use jvm_throwable::JvmThrowable;