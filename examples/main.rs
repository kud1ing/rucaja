extern crate jni_sys;
extern crate rucaja;

use jni_sys::{JNI_FALSE, JNI_TRUE};
use rucaja::{jvalue_from_jboolean, Jvm, JvmClass};
use std::ptr::null;


fn call_static_boolean_method(jvm: &Jvm, jvm_class: &JvmClass) {

    unsafe {
        let args = vec![jvalue_from_jboolean(JNI_FALSE)];
        let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethod", "(Z)Z").expect("Could not find JVM method");
        let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method, args.as_ptr());
        println!("`call_static_boolean_method()`; {:?}", result);

        let args = vec![jvalue_from_jboolean(JNI_TRUE)];
        let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethod", "(Z)Z").expect("Could not find JVM method");
        let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method, args.as_ptr());
        println!("`call_static_boolean_method()`; {:?}", result);
    }
}

fn call_static_object_method(jvm: &Jvm, jvm_class: &JvmClass) {

    unsafe {
        let jvm_method = jvm.get_static_method(&jvm_class, "staticObjectMethod", "()Ljava/lang/String;").expect("Could not find JVM method");
        let result = jvm.call_static_object_method(&jvm_class, &jvm_method, null());
        println!("`call_static_object_method(): {:?}`", result);

        // Print the object.
        //let args = vec![result];
        //jvm.call_static_void_method(&jvm_class, &jvm_println_method, args.as_ptr());
    }
}

fn call_static_void_method(jvm: &Jvm, jvm_class: &JvmClass) {

    unsafe {
        let jvm_method = jvm.get_static_method(&jvm_class, "staticVoidMethod", "()V").expect("Could not find JVM method");
        jvm.call_static_void_method(&jvm_class, &jvm_method, null());
        println!("`call_static_void_method()`");
    }
}


fn main() {
    let jvm_options = [
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        let jvm = Jvm::new(&jvm_options);
        let jvm_class = jvm.get_class("Test").expect("Could not find JVM class");

        let jvm_system_class = jvm.get_class("java/lang/System").expect("Could not find JVM class");
        //let jvm_println_method = jvm.get_static_method(&jvm_class, "println", "()(Ljava/lang/Object;)V").expect("Could not find JVM method");

        call_static_boolean_method(&jvm, &jvm_class);
        call_static_object_method(&jvm, &jvm_class);
        call_static_void_method(&jvm, &jvm_class);
    }
}
