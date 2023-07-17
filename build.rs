use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .dynamic_library_name("util")
        .ctypes_prefix("libc")
        .raw_line("use libc::pid_t;")
        .blocklist_type(".*pid_t")
        .allowlist_var("UTMP_FILE")
        .allowlist_var("UTMP_FILENAME")
        .allowlist_var("WTMP_FILE")
        .allowlist_var("WTMP_FILENAME")
        .allowlist_function("login_tty")
        .allowlist_function("login")
        .allowlist_function("logout")
        .allowlist_function("logwtmp")
        .allowlist_function("updwtmp")
        .allowlist_function("utmpname")
        .allowlist_function("getutent")
        .allowlist_function("setutent")
        .allowlist_function("getutid")
        .allowlist_function("getutline")
        .allowlist_function("pututline")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
