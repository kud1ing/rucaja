use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jlongArray, jsize};

jvm_array_wrapper!(JvmLongArray, jlongArray);
