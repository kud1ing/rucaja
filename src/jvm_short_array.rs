use jni_sys::{ jshortArray, jsize };
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmShortArray, jshortArray);
