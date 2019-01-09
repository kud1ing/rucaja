use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jbyteArray, jsize};

jvm_array_wrapper!(JvmByteArray, jbyteArray);
