use jni_sys::jbyteArray;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper_struct!(JvmByteArray, jbyteArray);