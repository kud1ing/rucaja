use jni_sys::jobject;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmObject, jobject);