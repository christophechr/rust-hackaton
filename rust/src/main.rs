//fn element_at(slice: &[i32], index: usize) -> i32 {
//    return slice[index]
//}
//
//fn slices_len(slice: &[i32]) -> usize {
//    return slice.len()
//}

//fn evaluate_number(number : i32) {
//    if number > 0 {
//        println!("Positive");
//    } else if number < 0 {
//        println!("Negative");
//    } else {
//        println!("Zero");
//    }
//}

//fn print_slices_element(slice: &[String]) {
//    for text in slice {
//        println!("{}", text);
//    }
//}

//fn add(x: i32, y: i32) -> i32 {
//    return x + y;
//}
//
//fn print_modulo(x: i32, y: i32) {
//    println!("{}", x%y);
//}
//
//fn slice_sum(slice: &[i32]) -> i32 {
//    let mut sum = 0;
//    for value in slice {
//        sum += value;
//    }
//    return sum
//}

//struct School {
//    school_name: String,
//    location: String,
//    address: String,
//    is_rated: bool,
//    rate: i32,
//}
//
//fn get_school_name(school: School) -> String {
//    return school.school_name;
//}

//enum Role {
//    Student,
//    Teacher,
//    Admin,
//}

//fn print_counter(counter : &u32) {
//    println!("Counter value : {}", counter);
//}
//
//fn increment_counter(counter: &mut u32) {
//    *counter += 1;
//}

//pub struct Animal {
//    race: String,
//    name: String,
//    age: i32,
//}

//pub struct Dog {
//    race: String,
//    name: String,
//    age: i32,
//}
//
//pub trait Getter {
//    fn get_race(&self) -> &String;
//
//    fn get_name(&self) -> &String;
//}

//impl Getter for Animal {
//    fn get_race(&self) -> &String {
//        return &self.race;
//    }
//
//    fn get_name(&self) -> &String {
//        return &self.name;
//    }
//}
//
//impl Getter for Dog {
//    fn get_race(&self) -> &String {
//        return &self.race;
//    }
//
//    fn get_name(&self) -> &String {
//        return &self.name;
//    }
//}

//fn last_element<T>(slice: &[T]) -> &T {
//    return &slice[slice.len() - 1];
//}

//fn func() {
//    let closure = |slice : &[u32]| {
//        return (slice[0], slice[slice.len() - 1]);
//    };
//
//    println!("{:?}", closure(&[1, 2, 3, 4, 5, 6]));
//
//}

//mod book_mod {
//    pub struct Book {
//        name: String,
//    }
//    impl Book {
//        pub fn new(book_name: &str) -> Book {
//            return Book {name: book_name.to_string()};
//        }
//    }
//    pub fn read_book(book: &Book) {
//        println!("Reading book: {}", book.name);
//    }
//}

fn is_none(option: Option<i32>) -> bool {
    return false;
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}
