use jni_sys::{ jarray, jsize };
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmArray, jarray);
