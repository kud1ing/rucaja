use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jcharArray, jsize};

jvm_array_wrapper!(JvmCharArray, jcharArray);
