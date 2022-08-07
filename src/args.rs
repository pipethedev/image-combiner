use std::env;

fn get_nth_arg(n: usize) -> String {
    let args: Vec<String> = env::args().collect();
    return args[n].clone();
}

#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        return Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }  
    }
}