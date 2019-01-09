extern crate jni_sys;
extern crate rucaja;

use rucaja::{
    jvalue_from_jboolean, jvalue_from_jobject, Jvm, JvmAttachment, JvmClass, JvmMethod, JvmString,
    JNI_FALSE, JNI_TRUE,
};
use std::ptr::null;

/// The main function.
fn main() {
    // JVM options.
    let jvm_options = [
        //"-Djava.security.debug=all",
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    // Instantiate the embedded JVM.
    let jvm = Jvm::new(&jvm_options);

    // Attach the current native thread to the JVM.
    let jvm_attachment = JvmAttachment::new(jvm.jvm()).unwrap();

    // Get the Java class `Test` from `Test.class`.
    let class = JvmClass::get_class(&jvm_attachment, "Test").expect("Could not find JVM class");

    // Get the `println()` method from `Test.class` so that we can print JVM objects.
    let println =
        JvmMethod::get_static_method(&jvm_attachment, &class, "println", "(Ljava/lang/Object;)V")
            .expect("Could not find JVM method");

    create_a_java_string(&jvm_attachment, &class, &println);

    // Call some Java methods from `Test.class`.
    call_static_boolean_method(&jvm_attachment, &class);
    call_static_object_method(&jvm_attachment, &class, &println);
    call_static_void_method(&jvm_attachment, &class);
}

/// Calls a static Java method that returns a Java `bool`.
fn call_static_boolean_method(jvm_attachment: &JvmAttachment, class: &JvmClass) {
    // The arguments for the Java methods.
    let jvm_method_arguments = [JNI_FALSE, JNI_TRUE];

    // Get the Java method.
    let jvm_method = JvmMethod::get_static_method(
        &jvm_attachment,
        &class,
        "static_method_that_returns_a_boolean",
        "(Z)Z",
    )
    .expect("Could not find JVM method");

    // Iterate over the Java method arguments.
    for jvm_method_argument in &jvm_method_arguments {
        let args = vec![jvalue_from_jboolean(*jvm_method_argument)];

        // Call the java method with the current argument.
        let result = JvmMethod::call_static_boolean_method(
            &jvm_attachment,
            &class,
            &jvm_method,
            args.as_ptr(),
        );

        // Print the result of the Java call.
        println!(
            "* `call_static_boolean_method({})`; {:?}",
            jvm_method_argument, result
        );
    }
}

/// Calls static Java methods that return a Java object.
fn call_static_object_method(
    jvm_attachment: &JvmAttachment,
    class: &JvmClass,
    println: &JvmMethod,
) {
    // The names of Java methods to call.
    let jvm_method_names = [
        "static_method_that_returns_a_string",
        "static_method_that_returns_an_interned_string",
    ];

    // Iterate over the Java method names.
    for jvm_method_name in &jvm_method_names {
        // Get the current Java method.
        let jvm_method = JvmMethod::get_static_method(
            &jvm_attachment,
            &class,
            jvm_method_name,
            "()Ljava/lang/String;",
        )
        .expect("Could not find JVM method");

        // Call the Java method.
        let jvm_object =
            JvmMethod::call_static_object_method(&jvm_attachment, &class, &jvm_method, null())
                .unwrap();
        println!(
            "* `call_static_object_method(): `{}()` returned {:?}`",
            jvm_method_name,
            jvm_object.jvm_ptr()
        );

        // Print the Java result object via a Java method.
        println!("** print the JVM object:");
        let args = vec![jvalue_from_jobject(jvm_object.jvm_ptr())];
        JvmMethod::call_static_void_method(&jvm_attachment, &class, &println, args.as_ptr());
    }
}

/// Calls a static void Java method.
fn call_static_void_method(jvm_attachment: &JvmAttachment, class: &JvmClass) {
    // Get the Java method.
    let jvm_method =
        JvmMethod::get_static_method(&jvm_attachment, &class, "static_void_method", "()V")
            .expect("Could not find JVM method");

    JvmMethod::call_static_void_method(&jvm_attachment, &class, &jvm_method, null());
    println!("* `call_static_void_method()`");
}

/// Creates a Java string.
fn create_a_java_string(jvm_attachment: &JvmAttachment, class: &JvmClass, println: &JvmMethod) {
    println!("* `create_a_java_string()`");

    // Create a Java string.
    let jvm_string =
        JvmString::new(&jvm_attachment, "Hello World").expect("Could not create a string");

    // Print the Java string via a Java method.
    println!("** print the JVM string:");
    let args = vec![jvalue_from_jobject(jvm_string.jvm_ptr())];
    JvmMethod::call_static_void_method(&jvm_attachment, &class, &println, args.as_ptr());
}
