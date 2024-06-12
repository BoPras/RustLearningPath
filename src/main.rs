use std::{io::{self, Stdin}, string};

fn main() {
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



