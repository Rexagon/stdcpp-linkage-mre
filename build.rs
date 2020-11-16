fn main() {
    let dst = cmake::Config::new("./")
        .build();

    //println!("cargo:rustc-link-lib=dylib=stdc++"); // doesn't work when there

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=mylib");

    println!("cargo:rustc-link-lib=dylib=stdc++"); // works when there
}
