struct Person {
    name: String,
    _age: u32,
}

impl Person {
    fn new(name: String, _age: u32) -> Self {
        Self {name, _age}
    }

    fn greet(&self) {
        println!("Hello my name is {}", self.name);
    }
}

fn main () {
    let anish = Person::new(String::from("Anish"), 25);
    anish.greet();
}