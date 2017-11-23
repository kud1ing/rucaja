#![allow(dead_code)] // FIXME Remove this later.

/// Utility enum for representing JVM method types.
/// TODO Move this somewhere better so that it's more descriptive.
pub enum JType {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    /// On the left is the package componenets, on the right is the class name itself.
    Class(Vec<String>, String),
    /// Makes it an array of the type.
    Array(Box<JType>) // Boxed because sizing is silly.
}

impl Into<String> for JType {
    fn into(self) -> String {
        use self::JType::*;
        match self {
            Boolean => "Z".into(),
            Byte => "B".into(),
            Char => "C".into(),
            Short => "S".into(),
            Int => "I".into(),
            Long => "J".into(),
            Float => "F".into(),
            Double => "D".into(),
            Class(pkg, cls) => {
                let mut pp = String::new();
                for n in pkg {
                    pp.push_str(n.as_str());
                    pp.push('/');
                }
                format!("L{}{};", pp, cls)
            },
            Array(at) => {
                let jt: JType = *at; // It's yelling at me about type annotations.
                let s: String = jt.into();
                format!("[{}", s)
            }
        }
    }
}

/// Computes the JVM name for the method type specified by the arguments.
pub fn compute_jvm_method_signature(ret: Option<JType>, args: Vec<JType>) -> String {

    let mut a = String::new();
    for arg in args {
        let rep: String = arg.into(); // Still complaining about type annotations.
        a.push_str(rep.as_str());
    }

    let rrep: String = ret.map_or("".into(), |t| t.into());
    format!("({}){}", a, rrep)

}

// TODO Write a macro that wraps the call to compute_jvm_method_signature, because it's still ugly.

#[cfg(test)]
mod tests {

    use super::JType;
    use super::compute_jvm_method_signature;

    #[test]
    fn jvm_method_test_1() {
        // void foo();
        assert_eq!(compute_jvm_method_signature(None, vec![]), String::from("()"))
    }

    #[test]
    fn jvm_method_test_2() {
        // long f (int n, String s, int[] arr); // (from the JNI docs)
        assert_eq!(
            compute_jvm_method_signature(
                Some(JType::Long),
                vec![
                    JType::Int,
                    JType::Class(vec!["java".into(), "lang".into()], "String".into()),
                    JType::Array(Box::new(JType::Int))]),
            String::from("(ILjava/lang/String;[I)J"))
    }

}
