use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jarray, jsize};

jvm_array_wrapper!(JvmArray, jarray);
