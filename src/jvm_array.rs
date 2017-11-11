use jni_sys::jarray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmArray, jarray);
