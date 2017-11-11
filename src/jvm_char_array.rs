use jni_sys::{ jcharArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmCharArray, jcharArray);
