use std::io; // для открытие потока приема информации
use std::io::Write; //что бы можно было flush принта без переноса строк


fn main() {

    println!("\n Калькулятор дискриминанта (v2.0) \n\n ✳️  Github: https://github.com/alexzai007\n ✳️  Telegram: https://t.me/Alexzai007best\n");

    //запускаем бесконечный цикл
    loop {
        let mut a  = String::new();
        let mut b = String::new();
        let mut c = String::new();

        //вводим красиво параметр 
        print!("📥 Введите параметр a: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut a)
        .expect("⛔ Не получилось прочитать строку");     
        let a: f64 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⛔ Ошибка ввода даных, цикл перезапущен!\n");
                continue;
            },
        };

        //вводим красиво параметр 
        print!("📥 Введите параметр b: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut b)
            .expect("⛔ Не получилось прочитать строку");
        let b: f64 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⛔ Ошибка ввода даных, цикл перезапущен!\n");
                continue;
            },
        };

        //вводим красиво параметр 
        print!("📥 Введите параметр c: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut c)
            .expect("⛔ Не получилось прочитать строку");
        let c: f64 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⛔ Ошибка ввода даных, цикл перезапущен!\n");
                continue;
            },
        };

        //находим дискреминант и записываем его в отдельную переменую
        let discriminant: f64 = (b*b)-(4.0*(a*c));
        println!("\n☑️ Найдем дискриминант по формуле: \n\n ▫️ D = b^2 - 4 * a * c");
        println!(" ▫️ D = {}^2 - 4 * {} * {}", b, a, c);
        println!(" ▫️ D = {} - 4 * {}", b*b, a*c);
        println!(" ▫️ D = {} - {}", b*b, 4.0*a*c);
        println!(" ▫️ D = {}\n", discriminant);

        //находим иксы если они есть
        if discriminant > 0.0{
            println!("⁉️ Дискрименант положительный! Из этого слудует что корня два!\n");
            println!("☑️ Найдем корни по формуле:\n\n ▫️ x = -b ± √D / 2 * a");
            let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
            println!(" 🔸 x1 = {}\n 🔸 x2 = {}\n", x1, x2);

        }else if discriminant == 0.0{
            println!("⁉️ Дискрименант равен нулю! Из этого слудует что корень один!\n");
            println!("☑️ Найдем корни по формуле:\n\n ▫️ x = -b ± √D / 2 * a");
            let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
            println!(" 🔸 x = {}\n", x1);
        
        }else{
            println!("⚠️ Дискрименант отрицательный! Из этого слудует что корни комплексные числа!\n")
        }



    }

}
