use assert_cmd::Command;

#[test]
fn cli() {
    let mut cmd = Command::cargo_bin("rust-cli").unwrap();
    println!(">>>{:?}", cmd.output());
}
