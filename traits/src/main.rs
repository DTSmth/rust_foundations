use std::any::Any;
use std::fmt;
use std::fmt::Formatter;

struct Point {
    x: i32,
    y: i32,
}

trait Animal: std::fmt::Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow")
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("woof")
    }
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
    println!("{animal:?}")
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    // fn speak() {
    //     println!("no idea")
    // }
    fn as_any(&self) -> &dyn Any;

}

struct Tortoise;

impl DowncastableAnimal for Tortoise {

    fn as_any(&self) -> &dyn Any {
        self
    }
}



fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
    speak_twice(&cat);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    animals.iter().for_each(|animal| animal.speak());

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];

}
