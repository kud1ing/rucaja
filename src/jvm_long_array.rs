use jni_sys::jlongArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmLongArray, jlongArray);