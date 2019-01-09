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
mod jvm_type_signature;

pub use jni_sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jdouble, jdoubleArray,
    jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jobjectArray, jshort,
    jshortArray, jthrowable, jvalue, JNIEnv, JNI_FALSE, JNI_TRUE,
};

pub use crate::jvm::{
    jvalue_from_jboolean, jvalue_from_jbyte, jvalue_from_jchar, jvalue_from_jdouble,
    jvalue_from_jfloat, jvalue_from_jint, jvalue_from_jlong, jvalue_from_jobject,
    jvalue_from_jshort, Jvm,
};
pub use crate::jvm_array::JvmArray;
pub use crate::jvm_attachment::JvmAttachment;
pub use crate::jvm_boolean_array::JvmBooleanArray;
pub use crate::jvm_byte_array::JvmByteArray;
pub use crate::jvm_char_array::JvmCharArray;
pub use crate::jvm_class::JvmClass;
pub use crate::jvm_double_array::JvmDoubleArray;
pub use crate::jvm_float_array::JvmFloatArray;
pub use crate::jvm_int_array::JvmIntArray;
pub use crate::jvm_long_array::JvmLongArray;
pub use crate::jvm_method::JvmMethod;
pub use crate::jvm_object::JvmObject;
pub use crate::jvm_object_array::JvmObjectArray;
pub use crate::jvm_short_array::JvmShortArray;
pub use crate::jvm_string::JvmString;
pub use crate::jvm_throwable::JvmThrowable;
