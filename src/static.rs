trait Walkable {
    fn walk(&self);
}

struct Cat;
struct Dog;

impl Walkable for Cat {
    fn walk(&self) {
        println!("Cat is walking");
    }
}

impl Walkable for Dog {
    fn walk(&self) {
        println!("Dog is walking");
    }
}


fn generic_walk<T: Walkable>(t: T) {
    t.walk();
}

fn main() {
    let cat = Cat{};
    let dog = Dog{};

    generic_walk(cat); // Cat is walking
    generic_walk(dog); // Dog is walking
}