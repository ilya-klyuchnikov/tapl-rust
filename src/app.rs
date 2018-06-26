extern crate lib;
use lib::arith::*;
use lib::arith::Command::*;

extern crate serde;
extern crate serde_json;

fn main() {
    let args = [
        "-cp",
        "../tapl-scala/jvm/target/scala-2.12/taplScala-assembly-0.1.0-SNAPSHOT.jar",
        "tapl.ArithDemo",
        "../tapl-scala/examples/arith.tapl",
        "--dump-ast",
    ];
    let output = std::process::Command::new("java")
        .args(&args)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("stdout: {}", stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    let commands: Vec<Command> = serde_json::from_str(&stdout).unwrap();
    println!("parsed: {:?}", commands);

    for command in commands {
        match command {
            EvalCommand {term} => {
                println!("{:?}", &term);
                println!("{}", "||");
                println!("{}", "\\/");
                let result = big_step_evaluator::eval(*term);
                println!("{:?}", &result);
            }
        }
    }
}
