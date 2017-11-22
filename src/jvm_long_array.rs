use jni_sys::{ jlongArray, jsize };
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmLongArray, jlongArray);
