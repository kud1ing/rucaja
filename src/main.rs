extern crate rucaja;

use rucaja::Jvm;


fn main() {

    let jvm = Jvm::new();

    // Get the Java class.
    let java_class = jvm.find_class("Test");

    // Get the Java method ID.
    let java_method_id = jvm.get_static_method_id(&java_class, "helloRust", "()V");

    // Call the Java static void method.
    jvm.call_static_void_method(&java_class, &java_method_id);
}
