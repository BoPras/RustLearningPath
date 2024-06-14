use std::{collections::btree_map::Values, io::{self, Stdin}, string};

fn main() {
    //cargo run, cargo build, cargo build --release

    println!("How was your day");
    println!("Please input your feeling");
    let mut feel = String::new();

    io::stdin()
        .read_line(&mut feel)
        .expect("Failed to read line");

    println!(" I am feeling {}", feel);
}

/* ini coba unit test */
#[test]
fn hello_test(){
    let msg = "Happy Birthday";
    let name = "Amika";
    let age = 24;
    println!("{} yang ke-{}, {}",msg,age,name); //result statement
}
#[test]
fn number_operation(){
        /* number(integer & float) conversion */
        let a = 30;
        let b: i64 = a as i64;
    
        let money = 10000.3;
        println!("{}",money as f32);
        println!("{}",b) ;
    
        let mut numberA = 10;
        numberA += 10;
        println!("{}", numberA);
    
        numberA -= 20;
        println!("{}", numberA);
}

#[test]
fn comparison(){
    let mut status: bool = true;
    let a = 20;
    let b = 30;
    
    status = a == b;
    println!("{}", status);
}

#[test]
fn boolean_operator(){
    let nilai_sql = 90;
    let nilai_csharp = 80;
    let nilai_dotnet = 80;

    let lulus_sql:bool = nilai_sql >= 70;
    let lulus_csharp:bool = nilai_csharp >= 80;
    let lulus_dotnet:bool = nilai_dotnet >= 80;

    let result:bool = lulus_csharp && lulus_dotnet && lulus_sql;
    println!("{}", result);
}

#[test]
fn tuple(){
    let data: (i32,bool,char,f64) = (10,true,'A',10.5);
    println!("{:?}", data);
    println!("{:?}", data.2); //index ke 0,1,2,3

    let (a , b , _, d) = data; // gunakan _ untuk mengeliminasi index yang tidak kita mau
    println!("{} {} {}", a,b,d);

    let mut animal: (&str,&str,i32,bool) = ("Cat","Jakarta",5,true);
    animal.0 = "Dog";
    println!("{}", animal.0);
}

#[test]
fn array(){
    let mut array=[0,1,3,4];
    array[2] = 10;
    println!("{}", array[2]);

    let length = array.len();
    println!("{}", length);

    let array2 = [[1,2,3],[4,5,6]];
    println!("{}",array2[1][0]);
}

#[test]
fn string_owned(){
    let mut text = "Apa kabar";
    text = "halo";
    printstr(text);

    let mut dynamic_string = String::from("Hello World");
    printstr(&dynamic_string);

    let string_slice: &str = &dynamic_string;

    dynamic_string.push_str(". It's awesome");
    println!("{}", dynamic_string);
    printstr(&dynamic_string);

    fn printstr(string: &str){
        println!("{}", string);
    }
}
#[test]
fn ownership(){
    let name1: String = String::from("Pras");   //store a heap value
    println!("{}",name1);
    let name2 = name1; //ownership change, name1 no longer exist
    let name3 = name2.clone(); //clone the value, each value has individual owner

    printstr(name2, name3);
    fn printstr(string1:String, string2:String){
        println!("{},{}", string1,string2);
    }
}

#[test]
fn if_else_expression(){
    let score = 15;
    let result: &str;

    //if expression mengembalikan nilai
    //value dari if expression bisa disimpan di variable
    if score < 10 {
        result = "Bad";
        println!("{}",result);
    } else if score == 10 {
        result = "Good";
        println!("{}",result);
    } else {
        result = "Excellent";
        println!("{}",result);
    }

    let hari_ini = if result == "Bad"{
        "Sad"
    } else if result == "Good"{
        "Happy"
    } else if result == "Excellent"{
        "Wonderful"
    } else {
        "IDK"
    };

    println!("{}", hari_ini);
}

#[test]
fn loop_expression(){
    let mut counter = 0;

    loop {
        counter += 1;


        if counter == 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("counter: {}",counter);
    }
}

#[test]
fn while_loop(){
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("counter: {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["a","b","c","d","e"];
    let mut index = 0;

    while index < array.len() {
        print!("{}",array[index]);

        index += 1;
    }

    for value in array {
        println!("{}", value);
    }

    //collection: range variable
    let range1 = 0..5; //start inclusive, end exclusive
    print!("start: {}\n",range1.start);
    print!("end: {}\n",range1.end);

    for i in range1{
        println!("range: {}",i);
        println!("array-ke {}:{}",i,array[i]);
    }

    //range inclusive
    let range2 = 0..=5;
    print!("start: {}\n",range2.start());
    print!("end: {}\n",range2.end());

    for i in range2{
        println!("range_in: {}",i);
        // println!("array-ke_in {}:{}",i,array[i]);
    }

}

