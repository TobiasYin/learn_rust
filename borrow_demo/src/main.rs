struct Demo {
    v: Vec<u8>,
}

impl Demo {
    fn get_slice(&self, s: usize, e: usize) -> &str {
        return std::str::from_utf8(&self.v[s..e]).unwrap();
    }
    fn change_and_drop(&mut self, data: Vec<u8>) {
        self.v = data
    }
}

fn main() {
    let mut d: Demo = Demo { v: Vec::from("hello world") };
    let hello = d.get_slice(1, 2);
    println!("{}", hello);
    d.change_and_drop(Vec::from("new hello"));
    println!("{}", hello);
}
