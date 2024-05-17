use userprompt::{Password, Prompting};

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
enum TestEnum {
    Option1,
    Option2,
    Option3,
    Option4(String),
}

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
struct TestMe {
    e: TestEnum,
    bob: u8,
    jim: Option<u8>,
    asdf: TestMe2,
    pw: Password,
    path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
struct TestMe2 {
    size: u8,
    number: Option<u8>,
}

fn main() {
    println!("Running test program");
    println!("Please enter a value");
    let s = TestMe::prompt(None);
    if let Ok(s) = s {
        println!("You entered {:?}", s);
    }
}
