use cc;

fn main() {
    cc::Build::new()
        .cuda(true)
        .shared_flag(true)
        .flag("-allow-unsupported-compiler")
        .file("calculate.cu")
        .compile("libcalculate.so");
    println!("cargo:rustc-link-lib=calculate");
}


