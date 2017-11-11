use jni_sys::{ jobjectArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmObjectArray, jobjectArray);
