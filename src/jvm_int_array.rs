use jni_sys::jintArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmIntArray, jintArray);