mod cli;

fn main() {
    println!("Hello, world!");
    let args = cli::get_args();
    println!("{:#?}", args);
}
