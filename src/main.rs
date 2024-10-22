fn main() {
    println!("Hello, Đây là chương trình dùng để test flow");

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
}
