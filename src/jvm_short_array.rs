use jni_sys::{ jshortArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmShortArray, jshortArray);
