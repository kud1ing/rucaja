extern crate rucaja;

use rucaja::{jvalue_from_jobject, Jvm, JvmAttachment, JvmClass, JvmMethod, JvmString};

#[test]
fn test_java_integers() {
    let jvm = Jvm::new(&["-Xcheck:jni"]);

    // Attach the current native thread to the JVM.
    let jvm_attachment = JvmAttachment::new(jvm.jvm()).unwrap();

    let jvm_integer_class = JvmClass::get_class(&jvm_attachment, "java/lang/Integer").unwrap();

    let parse_int_method = JvmMethod::get_static_method(
        &jvm_attachment,
        &jvm_integer_class,
        "parseInt",
        "(Ljava/lang/String;)I",
    )
    .unwrap();

    for value in -500..500 {
        let jvm_string = JvmString::new(&jvm_attachment, value.to_string().as_str()).unwrap();

        let jvm_int = JvmMethod::call_static_int_method(
            &jvm_attachment,
            &jvm_integer_class,
            &parse_int_method,
            vec![jvalue_from_jobject(jvm_string.jvm_ptr())].as_ptr(),
        );

        assert_eq!(jvm_int, value);
    }
}
