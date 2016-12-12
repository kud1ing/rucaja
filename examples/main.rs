extern crate rucaja;

use rucaja::jvm::Jvm;

fn main() {
    let jvm = Jvm::new();

    let jvm_class = jvm.class("Test").expect("Could not find JVM class");
    let jvm_method = jvm.static_method(&jvm_class, "helloRust", "()V").expect("Could not find JVM method");

    jvm.call_static_void_method(&jvm_class, &jvm_method);
}
