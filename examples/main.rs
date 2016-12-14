extern crate rucaja;

use rucaja::Jvm;

fn main() {

    let jvm_options = ["-verbose:jni"];
    let jvm = Jvm::new(&jvm_options);

    let jvm_class = jvm.get_class("Test").expect("Could not find JVM class");
    let jvm_method = jvm.get_static_method(&jvm_class, "helloRust", "()V").expect("Could not find JVM method");

    jvm.call_static_void_method(&jvm_class, &jvm_method);
}
