use jni_sys::jclass;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmClass, jclass);
