use std::process::Command;

fn main() {
    let path = "ostep/chap4_cpu-intro/cpu-intro.pdf";

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path).spawn().unwrap();
}
