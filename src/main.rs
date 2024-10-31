mod executor;
use executor::{SingleExecutor, SingleExecutorType};

fn main() {
    let exec = SingleExecutorType::spawn(func());
    let ret = SingleExecutor::run(&exec);

    println!("{:#?}", ret);

    let exec = SingleExecutorType::spawn(func2());
    let ret = SingleExecutor::run(&exec);

    println!("{:#?}", ret);
}

async fn func() -> String {
    String::from("Hello World!")
}

async fn func2() -> i32 {
    111
}
