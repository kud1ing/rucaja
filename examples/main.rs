extern crate jni_sys;
extern crate rucaja;

use jni_sys::{JNI_FALSE, JNI_TRUE};
use rucaja::{jvalue_from_jboolean, Jvm};
use std::ptr::null;

fn main() {
    let jvm_options = [
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        let jvm = Jvm::new(&jvm_options);

        let jvm_class = jvm.get_class("Test").expect("Could not find JVM class");

        // Static boolean method.

        let args = vec![jvalue_from_jboolean(JNI_FALSE)];
        let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethod", "(Z)Z").expect("Could not find JVM method");
        let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method, args.as_ptr());
        println!("`call_static_boolean_method()`; {:?}", result);

        let args = vec![jvalue_from_jboolean(JNI_TRUE)];
        let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethod", "(Z)Z").expect("Could not find JVM method");
        let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method, args.as_ptr());
        println!("`call_static_boolean_method()`; {:?}", result);

        // Static void method.

        let jvm_method = jvm.get_static_method(&jvm_class, "staticVoidMethod", "()V").expect("Could not find JVM method");
        jvm.call_static_void_method(&jvm_class, &jvm_method, null());
        println!("`call_static_void_method()`");
    }
}
