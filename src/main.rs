// fn main() {
//                                          //            number              ///
//     // let x: i32 = -423;
//     // let y: u8 =  123;

//     // println!("Signed Integer:   {}", x);
//     // println!("Unsigned Integer: {}", y);
//                                          //          Floats Numbers       ///
//     // let floatP_Num: f32 = 12.32;
//     // let floatN_Num: f32 = -12.32;

//     // println!("this is the positive number : {floatP_Num}, and this is the negative number :{floatN_Num}");

//                                          //            Boolean            ///
 
//     //     const trueAss: bool = true;
//     //     const falseAss: bool = false;

//     //    println!("is true is :{}, and is false is :{}", trueAss , falseAss);

//                                          //            Char            /// 
//     // let first_alphabet: char = 'a'; 
//     //    print!("the first alphabet in English is :{}", first_alphabet);
    
//                                         //            String            ///
//     // let name: &str = "Mohammed";
//     //    print!("my  name is :{name}");
    
//     // let full_name =  String::from("benjamin alixander");
//     // print!("my full name is :{full_name}");


//                                         //            Arrays            ///

//     // let numbers: [i32; 5] = [1,2,4,5,6];

//     // println!("the first number is :{}", numbers[0]);
//     // println!("the third number is :{}", numbers[2]);
//     // println!("the hole numbers is :{:?}", numbers);
    

//     // let fruits: [&str; 3] = ["apple", "banana", "orange"];

//     // println!("we only have this fruits: {:?}", fruits);



//                                             //            tuples           ///

//     // let human: (&str,  i32, bool) = ("yanis", 30, true);
//     // let abus: String = ("benhah").to_string();
//     // println!("the humaan is : {:?}", human);
//     // println!("the abus is : {:?}", abus)
    
//     human_id(("Fethallah").to_string(), 21, true);
//     let a: i32 = addNum(2, -4);
//     println!("the sum is :{a}");
// }

// fn addNum(x: i32, y: i32) -> i32 {
//    x + y 
// }

// fn human_id(name: String, age: i32, is_male: bool) {
//     println!("My name is {name}, i am {age} and i am {is_male} man");
// }

// fn main() {
//     let s1 = String::from("BenMokhtar__Fethallah");
//     let len = calculate_length(&s1);
//     println!("the length of {}, is {} alphabet",s1 ,len);
// }
// fn calculate_length(s: &String) -> usize {
//    return  s.len();
// }

// fn main() {
//    let s1: String = String::from("LOHA");
//    let s2 = &s1 ;
//    println!("S1 is {} and S2 is {} .",s1, s2);
// }

// fn main() {
//     let mut x:  u32 = 10; 
//     x +=  23;
//     print!("x is :{}", x);
// }

// fn is_able(age: i32) -> bool {
//     let min_age = 18;
    
//     age >= min_age
// }

// fn main() {
//     let age = 20; // Example age value
//     let able = is_able(age);
    
//     println!("Is the person able? {}", able);
// }


// fn main() {
//     let min_age = 18;
//     let age = 2;
    
//     if age >= min_age {
//             println!("you are able to drink");
//     }
//     else{
//         println!("your are not able to drink");
//     } 
// }

// fn main() {
//     let mut x: i32 = 0;
//     while x < 10 {
//         print!("{}",x);
//         x += 1;
//     }
// }


// fn main() {
//     let mut x = 0;
//     loop {
//         if x == 10 {
//             break;
//         }
//         else if x < 10 {
//             print!("{}",x);
//             x += 1;
//         }
//     }
// }

// fn main() {
//     #[derive(Debug)]
//     struct Book {
//         title: String,
//         author: String,
//         pages: u32,
//     }

//     let book = Book {
//         title: String::from("The Law of Attraction"),
//         author: ("Fethallah").to_string(),
//         pages: 200,
//     };

//     println!("Book {:?}", book);

// }





// fn main() {
//     #[derive(Debug)]
//     enum  color  {
//        On,
//        Off
//     }
//     let light = color::On; 
//     println!("the light is : {:?}", light);
// }

// use rand::random;

// fn pick(value: u8) -> bool {
//     if value % 2 == 0 {
//         return  true;
//     } else {
//         return false;
//     }
// }

// fn main() {
//    let _v = rand::random();
   
//    println!("you picked {}, and the result is {}", _v, 
//    if pick(_v) {"positive"} else {"negative"});
// }




// fn main() {
//     let mut  positive_number: u32 = 42;
//     let negative_number: i32 = -3145;
//     let is_true:bool = true;
//     let mut is_false = false;
//     let mut  my_name: &str = "Fethallah";
//     let mut name : String = String::from("AYman");
//     let _pi : f32 = 3.14;
//     let _my_char : char = '🧱'; 
//     is_false = true;
//     positive_number = 457;
//     name = String::from("add");   
//     my_name = "bennes"; 
//     println!("my char is {}, my piositive number is :{} , and my negative number is :{}, is true is {}, and is false is {}, my name is {}, and name is {}",_my_char ,positive_number, negative_number, is_true, is_false, my_name, name);
// }


// fn main () {
//     let  days_of_week: [&str; 7] = ["m","t","w","t","f","s","s"];

//     let mut i = 0 ;

//     let the_first_element = days_of_week[0];
//     while i < days_of_week.len() {
//             println!("{}", days_of_week[i]);
//             i += 1;
//         } 
//     println!("the days of the week is {:?}, AND THE FIRST DAY IS {}, ", days_of_week, the_first_element);
//     } 

// tuples 
// fn main() {
//        let mut person = ("ben", 20 , true);
//        let name = person.0;
//        let age: u8 = person.1 + 20;
//        let available = person.2;
       
//        println!("the name is {name},and the age is {age} and he is {available} ");
//  }


// fn main () {
//     let sum_num = add(-7,14);
    
//     println!("the sum of _7 and 14 is :{}",  add(-7,14));
//     println!("the sum of _7 and 14 is :{}",  sum_num);

// }

// fn add(x: i32, y: i32) -> i32 {
//     x + y 
// }

// fn main () {
//     let mut counter = 0 ;

//     while counter < 5  {
//         println!("the number is : {}", counter);
//         counter += 1;
//     }
    
//     let nums : [i32; 5] = [1,2,3,4,5];
//     for counter in nums{
//         println!("the number is {}",counter);
//     }
// }


// fn main () {
//     let mut counter = 0; 

//     loop {
//         println!("the number is : {}", counter);
//         counter += 1; 

//         if counter > 6 {
//             break;
//         }
//     }
// }

// fn main () {
//     let num = 6;

//     match num {
//         1 => println!("numm is equal to one"),
//         2 => println!("num is equal to two"),
//         _ => println!("num is not equal both one and two")
//     }
// }


// ownerShip and Borrowing 

// fn main() {
//     let mut  my_string = String::from("Hello, rust!🪆");

//     print(&my_string);
//     change_string(&mut my_string);
//     println!("{}", my_string)
// }


// fn print(s: &String) {
//      println!("{}", s);
// }

// fn change_string(s: &mut String) {
//   s.push_str(" from mars ")
// }


// fn main() {
//     let original_string = String::from("Hello ,rust!");
//     let cloned_string = original_string.clone();

//     println!("the original string is {}", original_string );
//     println!("the cloned string is {}"  , cloned_string );

// }



// fn main () {
//       let  string_one = String::from("hello");
//       let  string_two = String::from(" World !");
// 
   //   let result = concatenate_strings(&string_one, &string_two);
//       println!("the result is {}", result);
// }

// fn concatenate_strings(s1: &str, s2: &str) -> String {
//       let result = s1.to_owned() + s2; 
//       return result; 
// // }
// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     pages: u32
// }

// fn main() {

//     let book = Book {
//         title: String::from("the law of attraction"),
//         author: String::from("manini"),
//         pages: 204
//     };

//     println!("{:?}",      get_book_info(book));
//     let book_two = create_book(String::from("catch the bug"), String::from("alisdov"), 231);
//     println!("{:?}", book_two);
// }

// fn get_book_info(book : Book) -> [String; 3] {
//     let title  = book.title;
//     let author = book.author;
//     let pages = book.pages;

//     let data =  [title, author, pages.to_string()];
//     return data; 
// }
// fn create_book(title: String, author: String, pages: u32) -> Book {
//     let title = title;
//     let author = author;
//     let pages = pages;
//     let book = Book {
//         title,
//         author,
//         pages
//     };
//     return book;
// }


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Method to calculate the area of the rectangle
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     // Associated function (not tied to an instance)
//     fn new(width: u32, height: u32) -> Self {
//         Self { width, height }
//     }
// }

// fn main() {
//     let rect = Rectangle::new(30, 50);
//     println!("The area of the rectangle is {} square pixels.", rect.area());
// }

// use std::os::windows::process;


// #[derive(Debug)]
// enum Wheather {
//     Sunny,
//     Cloudy,
//     Rainy,
//     Snowy,
//     Windy
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write (String),
//     ChangeColor(i32, i32, i32)
// }

// fn main() {
//      let message = Message::Write(String::from("hello world")); 
//      let message_two = Message::ChangeColor(32, 11, 2332);
//      process_message(message_two);
//     }

// fn process_message(msg: Message) {
//     match msg {
//         Message::Quit =>{
//             println!("Quit message");
//         } 
//         Message::Move { x, y } => {
//             println!("Move to ({} {})", x, y);
//         }
//         Message::Write(text) => {
//             println!("Written message: {}", text);
//         }
//         Message::ChangeColor( r, g ,b ) => {
//             println!("the color is ({} {} {})", r,g,b);
//         }
//     }
// }

// fn main() {
//     let x = 4.0;
//     let square_root = find_square_root(x);
 
//     match square_root {
//         Some(root) => {
//             println!("The square root of {} is {} ", x, root);
//         }
//         None => println!("the square of negative number is not defined")
//     }

//     let power_root: Option<f64> = find_power_root(x); 

//     match power_root {
//         Some(root) => {
//             println!("the power of {} is {}", x, root);
//         } 
//         None => print!("the power of 0 is 0")
//     }
// }
// fn find_square_root(number: f64) -> Option<f64> {
//     if number >= 0.0 {
//         Some(number.sqrt())
//     } else {
//         None
//     }
// }

// fn find_power_root(number: f64) -> Option<f64> {
//     if number == 0.0 {
//         None
//     } else  {
//         Some(number.powf(2.0))
//     } 
// }

// use std::vec;



// fn main () {

//     let _numbers = vec![1, 2, 3, 4];
//     let mut names : Vec<String>  = Vec::new();

//     names.push(String::from("fethallah")); // [fethallah]
//     names.push(String::from("imad")); // [fethallah, imad]

//     println!("the winners are {}", names[0]);
    

// } 

use::std::collections::HashMap;
fn main() {
      let mut scores = HashMap::new();

      scores.insert(String::from("ali"), 19);
}











































































































































































