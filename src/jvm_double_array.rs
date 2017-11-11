use jni_sys::{ jdoubleArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmDoubleArray, jdoubleArray);
