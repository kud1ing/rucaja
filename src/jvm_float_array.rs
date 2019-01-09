use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jfloatArray, jsize};

jvm_array_wrapper!(JvmFloatArray, jfloatArray);
