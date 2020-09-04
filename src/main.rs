fn main() {
    println!("Hello, world!");

    chap1();
}

fn chap1() {
    // let mut x = 5;
    //
    // println!("number is: {}", x);
    // x = 6;
    // println!("number is: {}", x);

    // let objectvie: Option<i32> = Some(0);
    // let objectvie: Option<i32> = Some(1);
    // let objectvie: Option<i32> = None;
    // match objectvie {
    //     Some(x) if x % 2 == 0 => println!("even number: {}", x),
    //     Some(x) => println!("odd number: {}", x),
    //     None => println!("no value"),
    // }

    // let mut v = vec![];
    // v.push(1);

    let dog = Dog {};
    let cat = Cat {};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn linespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn linespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat;

impl Animal for Cat {
    fn linespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.linespan());
    println!("Scientific name: {}", animal.scientific_name());
}