interface Print {
	print(): void;
}

class SimpleClass implements Print {
	private _id: number | undefined;

	readonly name: string; // only in the constructor

	static count = 0;

	constructor(id: number, _name: string) {
		this._id = id;
		this.name = _name;
	}

	get id(): number | undefined {
		return this._id;
	}

	set id(value: number) {
		this._id = value;
	}

	print(): void {
		console.log(`SimpleClass ${this.id} ${this.name} ${SimpleClass.count}`);
	}
}

const myClass = new SimpleClass(100, "Yo!");
myClass.id = 200;

SimpleClass.count++;

myClass.print();

abstract class Animal {
	// A concrete method with a default implementation
	eat(): void {
		console.log("This animal is eating.");
	}

	// Abstract method: must be implemented by derived classes
	abstract makeSound(): void;
}

class Dog extends Animal {
	// Implementing the abstract method
	makeSound(): void {
		console.log("Woof! Woof!");
	}
}

class Cat extends Animal {
	// Implementing the abstract method
	makeSound(): void {
		console.log("Meow!");
	}
}

const dog = new Dog();
dog.eat();
dog.makeSound();

const cat = new Cat();
cat.eat();
cat.makeSound();
