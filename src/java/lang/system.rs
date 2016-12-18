use jvm::Jvm;
use jvm_class::JvmClass;

///
pub struct SystemClass<'a> {
    class: JvmClass<'a>,
}

impl<'a> SystemClass<'a>  {

    ///
    pub unsafe fn new(jvm: &Jvm) -> Option<SystemClass> {

        // Try to find the JVM class.
        let maybe_class = jvm.get_class("java/lang/System");

        if maybe_class.is_none() {
            return None;
        }

        let class = maybe_class.unwrap();

        Some(
            SystemClass {
                class: class,
            }
        )
    }

}