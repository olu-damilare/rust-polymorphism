struct Cat;

impl Cat {
    fn walk(&self) -> (){
       println!("Cat is walking")
    }
}

struct Dog;

impl Dog {
    fn walk(&self) -> (){
       println!("Dog is walking")
    }
}

fn main() {
   let cat = Cat{};
   let dog = Dog {};
   cat.walk();
   dog.walk();
}
