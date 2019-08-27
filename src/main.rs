fn main() {
    task1();
    task2();
    task3();
    task4();
    task5();
    task6();
    task7(10, 20, 30);
    let result = task8(5.6, 2.4, 10.2);
    println!("{}",result);
    task9(95);
    task9(32);
    task9(62);
    task10(2020);
    task11();
    task12();
    task13(5);
}

fn task1() {
    let name = "PAKISTAN ZINDABAD";
    let length = name.len();
    println!("{}", name);
    println!("LENGTH OF PAKISTAN ZINDABAD : {}", length);
}

fn task2() {
    let num1: u64 = 85;
    let num2: i16 = -550;
    println!("{}", num1);
    println!("{}", num2);
}

fn task3() {
    let _float: f32 = 56.6;
    println!("{}", _float)
}

fn task4() {
    let _num1 = 76;
    let _num2 = 23;
    println!("x+y : {}", _num1 + _num2);
    println!("x-y : {}", _num1 - _num2);
    println!("x*y : {}", _num1 * _num2);
    println!("x/y : {}", _num1 / _num2);
    println!("x%y : {}", _num1 % _num2);
}

fn task5(){
    let int_array = [100,150,200,250,300];
    println!("{:?}",int_array);
    println!("{}",int_array[1]);
    println!("{}",int_array[3]);
}

fn task6(){
    let tuples = ("IOT","AI","CLOUD",500.65,8645,65.4);
    println!("{:?}",tuples);
    println!("{}",tuples.2);
    println!("{}",tuples.4);
    println!("{}",tuples.5);
}

fn task7(x:i32,y:i32,z:i32){
    println!("{}",x+y+z);
}

fn task8(x:f32,y:f32,z:f32)->f32{
    return x*y*z
}

fn task9(x:u32){
    let marks = x;
    if marks > 80 {
        println!("Greater than 80 - Grade A+");
    }
    else if marks > 70 && marks <= 80 {
        println!("Between 70 and 80 - Grade A");
    }
    else if marks > 60 && marks <= 70 {
        println!("Between 60 and 70 - Grade B");
    }
    else if marks > 50 && marks <= 60 {
        println!("Between 50 and 60 - Grade C");
    }
    else if marks > 40 && marks <= 50 {
        println!("Between 40 and 50 - Grade D");
    }
    else {
        println!("Below 40 - Grade F")
    }
}

fn task10(x:u32) {
    let year = x;
    if year%4==0 {
        println!("{} is a leap year!",year);
    }
    else {
        println!("{} is not a leap year!",year);
    }
}

fn task11() {
    for number in 2..13 {
        if number%2==0 {
            print!("{},",number);
        }
    }
    println!("...");
}

fn task12(){
    let mut number = 1;
    while number <= 11{
        if number%2 == 1 {
            print!("{},",number);
        }
        number=number+1;
    }
    println!("...");
}

fn task13(x:u32){
    let number = x;
    for table in 1..11{
        println!("{} * {} = {}",number,table,number*table);
    }
}