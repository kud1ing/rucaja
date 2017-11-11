use jni_sys::jthrowable;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_wrapper!(JvmThrowable, jthrowable);
