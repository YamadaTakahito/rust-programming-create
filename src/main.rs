fn main() {
    println!("Hello, world!");

    // chap1();
    chap2();
}

fn chap2() {
    let i8: i8 = 1;
    let i16: i16 = 1;
    let i32: i32 = 1;
    let i64: i64 = 1;
    let i128: i128 = 1;

    let u8: u8 = 1;
    let u16: u16 = 1;
    let u32: u32 = 1;
    let u64: u64 = 1;
    let u128: u128 = 1;

    // let f8: f8 = 10.1;
    let f32: f32 = 10.1;
    let f64: f64 = 10.1;

    let s1: String = String::from("hello world");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    let mut t = (1, "2");
    t.0 = 1;
    t.1 = "hoge";

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);
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