use std::env;

fn main() {

    // Try to determine the Java home directory so that we can linl to `libjvm`.
    let java_home =
        env::var("JAVA_HOME")
        .ok()
        .unwrap_or("/Library/Java/JavaVirtualMachines/jdk1.8.0_51.jdk/Contents/Home".to_string());

    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/server", java_home);

    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/amd64/server", java_home);

    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/i386/server", java_home);
}
