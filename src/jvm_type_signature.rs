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

// TODO: `Into<String>` or `ToString`?
impl Into<String> for JvmType {

    fn into(self) -> String {

        use self::JvmType::*;

        match self {
            Boolean => "Z".into(),
            Byte => "B".into(),
            Char => "C".into(),
            Short => "S".into(),
            Int => "I".into(),
            Long => "J".into(),
            Float => "F".into(),
            Double => "D".into(),
            Class{package_components: pkg, class_name: cls} => {
                let mut pp = String::new();
                for n in pkg {
                    pp.push_str(n.as_str());
                    pp.push('/');
                }
                format!("L{}{};", pp, cls)
            },
            Array(at) => {
                let jt: JvmType = *at; // It's yelling at me about type annotations.
                let s: String = jt.into();
                format!("[{}", s)
            }
        }
    }
}

/// Builds the JVM method signature string.
pub fn jvm_method_signature_string(
    jvm_method_argument_types: Vec<JvmType>,
    jvm_method_return_type: Option<JvmType>
) -> String {

    let mut a = String::new();

    for arg in jvm_method_argument_types {
        let rep: String = arg.into();
        a.push_str(rep.as_str());
    }

    let rrep: String = jvm_method_return_type.map_or("".into(), |t| t.into());

    format!("({}){}", a, rrep)
}


#[cfg(test)]
mod tests {

    use super::JvmType;
    use super::jvm_method_signature_string;

    #[test]
    fn test_jvm_methods() {

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
                        package_components: vec!["java".into(), "lang".into()],
                        class_name: "String".into()
                    },
                    JvmType::Array(Box::new(JvmType::Int))
                ],
                Some(JvmType::Long),
            ),
            String::from("(ILjava/lang/String;[I)J")
        );
    }
}
