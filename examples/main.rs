extern crate jni_sys;
extern crate rucaja;

use jni_sys::{JNI_FALSE, JNI_TRUE};
use rucaja::{jvalue_from_jboolean, jvalue_from_jobject, Jvm, JvmClass, JvmMethod};
use std::ptr::null;


/// Calls a static Java method that returns a Java `bool`.
fn call_static_boolean_method(jvm: &Jvm, class: &JvmClass) {

    // The arguments for the Java methods.
    let jvm_method_arguments = [
        JNI_FALSE,
        JNI_TRUE,
    ];

    unsafe {

        // Get the Java method.
        let jvm_method = jvm.get_static_method(
            &class,
            "static_method_that_returns_a_boolean",
            "(Z)Z"
        ).expect("Could not find JVM method");

        // Iterate over the Java method arguments.
        for jvm_method_argument in &jvm_method_arguments {

            let args = vec![jvalue_from_jboolean(*jvm_method_argument)];

            // Call the java method with the current argument.
            let result = jvm.call_static_boolean_method(&class, &jvm_method, args.as_ptr());

            // Print the result of the Java call.
            println!("* `call_static_boolean_method({})`; {:?}", jvm_method_argument, result);
        }
    }
}

/// Calls static Java methods that return a Java object.
fn call_static_object_method(jvm: &Jvm, class: &JvmClass, println: &JvmMethod) {

    // The names of Java methods to call.
    let jvm_method_names = [
        "static_method_that_returns_a_string",

        // An attempted access to an interned `String` gives `java.security.AccessControlContext@0`.
        "static_method_that_returns_an_interned_string"
    ];

    unsafe {
        // Iterate over the Java method names.
        for jvm_method_name in &jvm_method_names {

                // Get the current Java method.
                let jvm_method = jvm.get_static_method(
                    &class,
                    jvm_method_name,
                    "()Ljava/lang/String;"
                ).expect("Could not find JVM method");

                // Call the Java method.
                let result = jvm.call_static_object_method(&class, &jvm_method, null());
                println!("* `call_static_object_method(): `{}()` returned {:?}`", jvm_method_name, result);

                // Print the Java result object via a Java method.
                println!("** print the JVM object:");
                let args = vec![jvalue_from_jobject(result)];
                jvm.call_static_void_method(&class, &println, args.as_ptr());
            }
    }
}

/// Calls a static void Java method.
fn call_static_void_method(jvm: &Jvm, class: &JvmClass) {

    unsafe {

        // Get the Java method.
        let jvm_method = jvm.get_static_method(
            &class,
            "static_void_method",
            "()V"
        ).expect("Could not find JVM method");

        jvm.call_static_void_method(&class, &jvm_method, null());
        println!("* `call_static_void_method()`");
    }
}

/// Creates a Java string.
fn create_a_java_string(jvm: &Jvm, class: &JvmClass, println: &JvmMethod) {

    unsafe {
        println!("* `create_a_java_string()`");

        // Create a Java method.
        let java_string = jvm.new_jstring("Hello World");

        // Print the Java string via a Java method.
        println!("** print the JVM string:");
        let args = vec![jvalue_from_jobject(java_string)];
        jvm.call_static_void_method(&class, &println, args.as_ptr());

    }
}

fn main() {

    // JVM options.
    let jvm_options = [
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        // Instantiate the embedded JVM.
        let jvm = Jvm::new(&jvm_options);

        // Get the Java class `Test` from `Test.java`.
        let class = jvm.get_class("Test").expect("Could not find JVM class");

        // Get the `println()` Java wrapper method for debugging the JVM objects.
        let println = jvm.get_static_method(
            &class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find JVM method");

        create_a_java_string(&jvm, &class, &println);

        // Call some Java methods.
        call_static_boolean_method(&jvm, &class);
        call_static_object_method(&jvm, &class, &println);
        call_static_void_method(&jvm, &class);
    }
}
