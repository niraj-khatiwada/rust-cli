use std::process::Command;

fn main() {
    let mut ls = Command::new("ls");
    match ls.arg("-la").output() {
        Ok(rs) => {
            let stdout = String::from_utf8_lossy(&rs.stdout[..]);
            println!("{}", stdout)
        }
        Err(err) => panic!("{}", err),
    }
}
