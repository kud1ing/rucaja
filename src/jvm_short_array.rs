use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jshortArray, jsize};

jvm_array_wrapper!(JvmShortArray, jshortArray);
