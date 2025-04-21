// build.rs
use std::env;
use std::io::Write;
use std::path::PathBuf;

/// Windows 平台构建逻辑
fn build_windows() {
    // 编译 windows.cc
    let windows_file = "src/platform/windows.cc";
    cc::Build::new()
        .file(windows_file)
        .compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32"); // 链接 Windows 库
    println!("cargo:rerun-if-changed={}", windows_file);
}

/// Windows 资源文件编译（需启用 inline 特性）
#[cfg(feature = "inline")]
fn build_manifest() {
    if env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("res/manifest.xml");

        if let Err(e) = res.compile() {
            let _ = writeln!(&mut std::io::stderr(), "资源编译失败: {}", e);
            std::process::exit(1);
        }
    }
}

/// 强制指定 Opus 头文件路径
fn force_opus_include_path() {
    let vcpkg_root = env::var("VCPKG_ROOT").expect("VCPKG_ROOT 环境变量未设置！");
    let include_path = PathBuf::from(vcpkg_root)
        .join("installed")
        .join("x64-windows")
        .join("include");

    // 向编译器传递头文件搜索路径
    println!("cargo:include={}", include_path.to_str().unwrap());
    println!("cargo:rerun-if-env-changed=VCPKG_ROOT"); // 环境变量变化时重新构建
}

fn main() {
    // 强制指定 Opus 头文件路径
    force_opus_include_path();

    // Windows 平台构建
    build_windows();

    // 如果启用 inline 特性，编译资源文件
    #[cfg(feature = "inline")]
    build_manifest();

    // 标记 build.rs 自身为依赖项
    println!("cargo:rerun-if-changed=build.rs");
}
