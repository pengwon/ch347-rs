use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let lib_dir = if target.contains("windows") {
        if target.contains("x86_64") {
            "lib/windows/amd64"
        } else if target.contains("i386") {
            "lib/windows/i386"
        } else if target.contains("arm64") {
            "lib/windows/arm64"
        } else {
            panic!("Unsupported Windows target: {}", target);
        }
    } else if target.contains("linux") {
        if target.contains("x86_64") {
            "lib/linux/x64/static"
        } else if target.contains("aarch64") {
            "lib/linux/aarch64/static"
        } else if target.contains("arm-gnueabi") {
            "lib/linux/arm-gnueabi/static"
        } else if target.contains("arm-gnueabihf") {
            "lib/linux/arm-gnueabihf/static"
        } else if target.contains("mips32") {
            "lib/linux/mips32/static"
        } else if target.contains("mips64") {
            "lib/linux/mips64/static"
        } else {
            panic!("Unsupported Linux target: {}", target);
        }
    } else {
        panic!("Unsupported target: {}", target);
    };

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=ch347");
}
