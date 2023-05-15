use std::{io};

fn main() {
    

    print!(r#"
____  _             _       _             _      ___   ___ 
|    \|_|___ ___ ___|_|_____|_|___ ___ ___| |_   |_  | |   |
|  |  | |_ -|  _|  _| |     | |   | .'|   |  _|  |  _|_| | |
|____/|_|___|___|_| |_|_|_|_|_|_|_|__,|_|_|_|    |___|_|___|

by alexzai007    
"#);


    let mut enter:String = String::new();
    //создаем переменые
    let mut a_string:String = String::new();
    let mut b_string:String = String::new();
    let mut c_string:String = String::new();


    //получем входные значения
    println!("Введите a:");
    match io::stdin().read_line( &mut a_string){
        Ok(_) => {},
        Err(e) => {print!("Фатальная ошибка {}", e)}
    };

    println!("\nВведите b:");
    match io::stdin().read_line( &mut b_string){
        Ok(_) => {},
        Err(e) => {print!("Фатальная ошибка {}", e)}
    };

    println!("\nВведите c");
    match io::stdin().read_line( &mut c_string){
        Ok(_) => {},
        Err(e) => {print!("Фатальная ошибка {}", e)}
    };

    //переводим стринг в int
    let a: f64 = a_string.trim().parse().unwrap();
    let b: f64 = b_string.trim().parse().unwrap();
    let c: f64 = c_string.trim().parse().unwrap();

    //  b^2 - 4ac
    // Считаем дискриминант
    let d: f64 = (b*b) - 4.0 * a * c;

    // ( -b + d^0.5 ) / 2 * a
    // ( -b - d^0.5 ) / 2 * a
    // Считаем уравнения
    if d > 0.0 {
        let x1: f64 =  ((-b) + d.sqrt()) / ( 2.0 * a);
        let x2: f64 =  ((-b) - d.sqrt()) / (2.0 * a);
        println!("\n\n-_-_-_-_-_-_-_-_-");
        println!("Дискриминант: {}", d);
        println!("первый x: {}", x1);
        println!("первый x: {}", x2);
        println!("-_-_-_-_-_-_-_-_-\n\n");

    }else if d == 0.0 {
        let x1: f64 =  ((-b) + d.sqrt()) / ( 2.0 * a);
        println!("\n\n-_-_-_-_-_-_-_-_-");
        println!("Дискриминант: {}", d);
        println!("первый x: {}", x1);
        println!("-_-_-_-_-_-_-_-_-\n\n");

    }else {
        println!("\n\n-_-_-_-_-_-_-_-_-");
        println!("Дискриминант: {}", d);
        println!("Корней нет");
        println!("-_-_-_-_-_-_-_-_-\n\n");
    }


    //ждем нажатия Enter для завершения
    println!("\nEnter...");
    match io::stdin().read_line( &mut enter){
        Ok(_) => {},
        Err(e) => {print!("Фатальная ошибка {}", e)}
    }

}
