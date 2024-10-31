trait SoundMaker {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl SoundMaker for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl SoundMaker for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn main() {
    let animals: Vec<Box<dyn SoundMaker>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.make_sound();
    }
}
