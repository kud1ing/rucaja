extern crate rucaja;

use rucaja::Jvm;

fn main() {

    let jvm_options = [
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    let jvm = Jvm::new(&jvm_options);

    let jvm_class = jvm.get_class("Test").expect("Could not find JVM class");

    let jvm_method = jvm.get_static_method(&jvm_class, "staticVoidMethod", "()V").expect("Could not find JVM method");
    jvm.call_static_void_method(&jvm_class, &jvm_method);

    let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethodFalse", "()Z").expect("Could not find JVM method");
    let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method);
    println!("`call_static_boolean_method()`; {:?}", result);

    let jvm_method = jvm.get_static_method(&jvm_class, "staticBooleanMethodTrue", "()Z").expect("Could not find JVM method");
    let result = jvm.call_static_boolean_method(&jvm_class, &jvm_method);
    println!("`call_static_boolean_method()`; {:?}", result);
}
