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

fn dynamic_dispatch(w: &dyn Walkable) {
    w.walk();
}

fn main() {
    let human = &Human{};
    let dog = &Dog{};

    dynamic_dispatch(human); // Human is walking
    dynamic_dispatch(dog); // Dog is walking
}