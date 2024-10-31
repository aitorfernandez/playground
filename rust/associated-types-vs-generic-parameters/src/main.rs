// Associated Type
trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
}

struct IntegerContainer(Vec<i32>);

impl Container for IntegerContainer {
    type Item = i32;

    fn add(&mut self, item: Self::Item) {
        self.0.push(item);
    }
}

// Generic Type
struct GenericContainer<T>(Vec<T>);

impl<T> GenericContainer<T> {
    fn add(&mut self, item: T) {
        self.0.push(item);
    }
}

fn main() {
    let mut int_container = IntegerContainer(Vec::new());
    int_container.add(5);

    let mut generic_container: GenericContainer<i32> = GenericContainer(vec![]);
    generic_container.add(5);
}
