use std::process::Command;

fn main() {
    let mut cmd = Command::new("echo");
    cmd.arg("hello world");
    match cmd.output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout[..]);
            print!("{}", stdout.trim()) // echo hello world returns value with new line appended. So, we trim to just give exact output.
        }
        Err(err) => panic!("{}", err),
    }
}
