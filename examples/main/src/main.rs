use userprompt::{Password, Prompting};

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
enum TestEnum {
    #[PromptComment = "This is the first option"]
    Option1,
    #[PromptComment = "This is a second option"]
    Option2,
    #[PromptComment = "This is another option"]
    Option3,
    #[PromptComment = "This is an option"]
    Option4(String),
    Option5 {
        #[PromptComment = "This is the first value representing nothing useful"]
        asdf: u32,
        #[PromptComment = "This is the second value representing stuff"]
        fdsa: u64,
    },
}

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
struct TestMe {
    #[PromptComment = "This is a plain vector of bytes"]
    vec1: Vec<u8>,
    #[PromptComment = "This is a vector of test enums"]
    vec2: Vec<TestEnum>,
    #[PromptComment = "This is a vector of test2 enums"]
    vec3: Vec<TestMe2>,
    #[PromptComment = "This is a simple test enum"]
    e: TestEnum,
    #[PromptComment = "This is a value ranging 0-255"]
    bob: u8,
    #[PromptComment = "This is an optional value ranging from 0-255"]
    jim: Option<u8>,
    #[PromptComment = "This is a struct for doing stuff"]
    asdf: TestMe2,
    #[PromptComment = "This is the magic password"]
    pw: Password,
    #[PromptComment = "This is a path that may or may not exist"]
    path: std::path::PathBuf,
    #[PromptComment = "Please enter a filename that exists"]
    fo: userprompt::FileOpen,
    #[PromptComment = "Please enter a filename to crate"]
    fc: userprompt::FileCreate,
}

#[allow(dead_code)]
#[derive(Debug, userprompt::Prompting)]
struct TestMe2 {
    #[PromptComment = "Please enter a size ranging from 0 - 255"]
    size: u8,
    #[PromptComment = "Please optionally enter a number ranging from 0 to 255"]
    number: Option<u8>,
}

fn main() {
    println!("Running test program");
    println!("Please enter a value");
    let s = TestMe::prompt(None, None);
    if let Ok(s) = s {
        println!("You entered {:?}", s);
    }
}
