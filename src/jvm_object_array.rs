use jni_sys::jobjectArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmObjectArray, jobjectArray);