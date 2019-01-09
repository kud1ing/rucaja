use crate::jvm_attachment::JvmAttachment;
use jni_sys::{jdoubleArray, jsize};

jvm_array_wrapper!(JvmDoubleArray, jdoubleArray);
