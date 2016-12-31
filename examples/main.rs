extern crate jni_sys;
extern crate rucaja;

use jni_sys::{JNI_FALSE, JNI_TRUE};
use rucaja::{jvalue_from_jboolean, jvalue_from_jobject, Jvm, JvmClass, JvmMethod};
use std::ptr::null;


fn call_static_boolean_method(jvm: &Jvm, class: &JvmClass) {

    let jvm_method_arguments = [
        JNI_FALSE,
        JNI_TRUE,
    ];

    unsafe {
        let jvm_method = jvm.get_static_method(
            &class,
            "static_method_that_returns_a_boolean",
            "(Z)Z"
        ).expect("Could not find JVM method");

        for jvm_method_argument in &jvm_method_arguments {
            let args = vec![jvalue_from_jboolean(*jvm_method_argument)];
            let result = jvm.call_static_boolean_method(&class, &jvm_method, args.as_ptr());
            println!("`call_static_boolean_method()`; {:?}", result);
        }
    }
}

fn call_static_object_method(jvm: &Jvm, class: &JvmClass, println: &JvmMethod) {

    // An attempted access to an interned `String` gives `java.security.AccessControlContext@0`.
    let jvm_method_names = [
        "static_method_that_returns_a_string",
        "static_method_that_returns_an_interned_string"
    ];

    for jvm_method_name in &jvm_method_names {
        unsafe {
            let jvm_method = jvm.get_static_method(&class, jvm_method_name, "()Ljava/lang/String;").expect("Could not find JVM method");

            let result = jvm.call_static_object_method(&class, &jvm_method, null());
            println!("`call_static_object_method(): `{}()` returned {:?}`", jvm_method_name, result);

            println!("Print the JVM object:");
            let args = vec![jvalue_from_jobject(result)];
            jvm.call_static_void_method(&class, &println, args.as_ptr());
        }
    }
}

fn call_static_void_method(jvm: &Jvm, class: &JvmClass) {

    unsafe {
        let jvm_method = jvm.get_static_method(&class, "static_void_method", "()V").expect("Could not find JVM method");
        jvm.call_static_void_method(&class, &jvm_method, null());
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
        let class = jvm.get_class("Test").expect("Could not find JVM class");
        let println = jvm.get_static_method(&class, "println", "(Ljava/lang/Object;)V").expect("Could not find JVM method");

        call_static_boolean_method(&jvm, &class);
        call_static_object_method(&jvm, &class, &println);
        call_static_void_method(&jvm, &class);
    }
}
