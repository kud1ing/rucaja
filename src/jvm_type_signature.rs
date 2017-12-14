#![allow(dead_code)] // FIXME Remove this later.

/// Represents a JVM type.
pub enum JvmType {

    Array(Box<JvmType>),
    Boolean,
    Byte,
    Char,
    Class{package_components: Vec<String>, class_name: String},
    Double,
    Float,
    Int,
    Long,
    Short,
}


/// Builds the JVM type signature string.
fn jvm_signature_string(jvm_type: &JvmType) -> String {

    match *jvm_type {
        JvmType::Boolean => String::from("Z"),
        JvmType::Byte => String::from("B"),
        JvmType::Char => String::from("C"),
        JvmType::Short => String::from("S"),
        JvmType::Int => String::from("I"),
        JvmType::Long => String::from("J"),
        JvmType::Float => String::from("F"),
        JvmType::Double => String::from("D"),
        JvmType::Class{ref package_components, ref class_name} => {

            let mut class_signature_string = String::from("L");

            // Iterate over the package components.
            for package_component in package_components {
                class_signature_string.push_str(package_component.as_str());
                class_signature_string.push('/');
            }

            // Add the class name.
            class_signature_string.push_str(class_name);
            class_signature_string.push_str(";");

            class_signature_string
        },
        JvmType::Array(ref jvm_array_type) => {
            format!("[{}", jvm_signature_string(&jvm_array_type))
        }
    }
}

/// Builds the JVM method signature string.
pub fn jvm_method_signature_string(
    jvm_method_argument_types: Vec<JvmType>,
    jvm_method_return_type: Option<JvmType>
) -> String {

    let mut method_signature_string = String::from("(");

    // Iterate over the JVM method arguments.
    for jvm_method_argument_type in jvm_method_argument_types {
        let rep: String = jvm_signature_string(&jvm_method_argument_type);
        method_signature_string.push_str(rep.as_str());
    }

    method_signature_string.push_str(")");

    // Build the return type signature string.
    let return_type_signature: String = jvm_method_return_type.map_or(
        String::from(""),
        |t| jvm_signature_string(&t)
    );
    method_signature_string.push_str(&return_type_signature);

    method_signature_string
}


#[cfg(test)]
mod tests {

    use super::JvmType;
    use super::jvm_method_signature_string;

    #[test]
    fn test_jvm_method_signature_string() {

        assert_eq!(
            // `void foo()`
            jvm_method_signature_string(vec![], None),
            String::from("()")
        );

        assert_eq!(
            // `long f(int, String, int[])`
            jvm_method_signature_string(
                vec![
                    JvmType::Int,
                    JvmType::Class{
                        package_components: vec![String::from("java"), String::from("lang")],
                        class_name: String::from("String")
                    },
                    JvmType::Array(Box::new(JvmType::Int))
                ],
                Some(JvmType::Long),
            ),
            String::from("(ILjava/lang/String;[I)J")
        );
    }
}
