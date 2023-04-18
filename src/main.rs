use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("1에서 100까지의 숫자 중 하나를 추리세요.");

    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽는 데 실패했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("잘못된 입력입니다.");
                continue;
            }
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("더 큰 수를 추리하세요!"),
            Ordering::Greater => println!("더 작은 수를 추리하세요!"),
            Ordering::Equal => {
                println!("축하합니다! 숫자를 맞췄습니다.");
                break;
            }
        }
    }
}