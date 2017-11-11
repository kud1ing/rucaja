use jni_sys::jcharArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmCharArray, jcharArray);