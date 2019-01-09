use crate::jvm_attachment::JvmAttachment;
use jni_sys::jthrowable;

jvm_wrapper!(JvmThrowable, jthrowable);
