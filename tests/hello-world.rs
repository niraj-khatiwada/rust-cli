use assert_cmd::Command;

#[test]
fn hello_world() {
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success().stdout("hello world");
}
