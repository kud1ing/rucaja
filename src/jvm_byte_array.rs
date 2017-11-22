use jni_sys::{ jbyteArray, jsize };
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmByteArray, jbyteArray);
