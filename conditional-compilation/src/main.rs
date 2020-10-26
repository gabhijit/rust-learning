#[cfg(target_arch = "x86")]
fn arch_x86_only() {
    println!("This will get compiled and printed when the arch is x86.");
}

fn main() {

    #[cfg(target_arch = "x86")]
    arch_x86_only();

    // On my linux following will get printed
    #[cfg(any(unix, target_pointer_width = "16", target_os = "macos"))]
    println!("Hello, world!");
}
