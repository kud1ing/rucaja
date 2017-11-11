use jni_sys::jbooleanArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmBooleanArray, jbooleanArray);
