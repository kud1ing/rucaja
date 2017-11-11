use jni_sys::{ jbooleanArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmBooleanArray, jbooleanArray);
