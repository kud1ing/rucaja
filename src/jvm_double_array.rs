use jni_sys::jdoubleArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmDoubleArray, jdoubleArray);