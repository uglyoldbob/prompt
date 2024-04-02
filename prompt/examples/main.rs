use prompt::{Password, Prompting};


#[allow(dead_code)]
#[derive(Debug, prompt::Prompting)]
struct TestMe {
    bob: u8,
    jim: Option<u8>,
    asdf: TestMe2,
    pw: Password,
    path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, prompt::Prompting)]
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