use jni_sys::{ jfloatArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmFloatArray, jfloatArray);
