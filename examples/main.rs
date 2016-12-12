extern crate rucaja;

use rucaja::jvm::Jvm;

fn main() {
    let jvm = Jvm::new();

    let jvm_class = jvm.class("Test").unwrap();

    let jvm_method_id = jvm.get_static_method_id(&jvm_class, "helloRust", "()V");

    jvm.call_static_void_method(&jvm_class, &jvm_method_id);
}
