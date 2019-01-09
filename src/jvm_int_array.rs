use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jintArray, jsize};

jvm_array_wrapper!(JvmIntArray, jintArray);
