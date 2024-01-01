fn main() {
    println!("Hello, world!");
}

fn parse_files<P: AsRef<Path>>(target: P) -> Result<Vec<Item>, Error> {}
