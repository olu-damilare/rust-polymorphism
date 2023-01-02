struct Human;

impl Human {
    fn walk(&self) -> (){
       println!("Human is walking")
    }
}

struct Dog;

impl Dog {
    fn walk(&self) -> (){
       println!("Dog is walking")
    }
}

fn main() {
   let human = Human{};
   let dog = Dog {};
   human.walk();
   dog.walk();
}
