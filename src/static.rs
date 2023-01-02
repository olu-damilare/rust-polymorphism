trait Walkable {
    fn walk(&self);
}

struct Human;
struct Dog;

impl Walkable for Human {
    fn walk(&self) {
        println!("Human is walking");
    }
}

impl Walkable for Dog {
    fn walk(&self) {
        println!("Dog is walking");
    }
}


fn static_dispatch<T: Walkable>(t: T) {
    t.walk();
}

fn main() {
    let human = Human{};
    let dog = Dog{};

    static_dispatch(&human); // Human is walking
    static_dispatch(&dog); // Dog is walking
}
