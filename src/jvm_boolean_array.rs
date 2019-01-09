use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jbooleanArray, jsize};

jvm_array_wrapper!(JvmBooleanArray, jbooleanArray);
