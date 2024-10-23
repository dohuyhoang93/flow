fn main() {
    println!("Hello, Đây là chương trình dùng để test flow với loop");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 1 {
                break;
            }
            
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {}", count);

    println!("Đây là mã test flow - while");
    let mut number = 3;

    while number != 0 {
        println!("số đã cho khác 0 là: {}!", number);

        number -= 1;
    }
    println!("LiftOff!!!");

    println!("Đây là code test flow - while với mảng");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { //cách tiếp cận dễ gây lỗi truy cập quá phạm vi
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("Đây là code test flow - for");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..=10).rev() {
        println!("the value is: {}", number);
    }
    println!("LiftOff");
}
