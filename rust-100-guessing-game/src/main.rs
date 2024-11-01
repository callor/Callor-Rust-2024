// rand crate 를 사용하기 위해 extern crate rand; 를 사용한다.
extern crate rand;

// io library is used to take input from the user
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // secret_number 변수를 생성하고, 1부터 100까지의 난수를 생성하여 저장한다.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        /*
        :new에 있는 ::는 new가 String 타입의 연관함수 임을 나타낸다. 
        연관함수는 하나의 타입을 위한 함수이며, 
        이 경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수이다. 
        몇몇 언어에서는 이것을 정적 메소드 라고 부른다.

        new 함수는 새로운 빈 String을 생성한다. 
        new 함수는 새로운 값을 생성하기 위한 일반적인 이름이므로 많은 타입에서 찾아볼 수 있다.

        요약하자면 let mut guess = String::new(); 라인은 새로운 빈 String 인스턴스와 연결된 가변변수를 생성한다.
        */
        let mut guess = String::new();


        /*
        io::stdin() 함수는 표준 입력 핸들에 대한 인스턴스를 반환한다.
        read_line 메소드는 사용자가 입력한 값을 가져와서 guess 변수에 저장한다.
        &는 참조자를 나타내는 연산자이다.
        참조자는 여러가지 이유로 인해 여러가지 방법으로 사용될 수 있다.
        여기서는 read_line 메소드에 참조자를 전달하여 사용자의 입력을 가져오는 것이다.
        참조자는 기본적으로 불변이기 때문에, guess를 가변으로 만들기 위해 mut 키워드를 사용한다.
        */ 

        /*
        만약 프로그램 시작점에 use std::io가 없다면 
        함수 호출 시 std::io::stdin처럼 작성해야 한다. 
        stdin 함수는 터미널의 표준 입력의 핸들(handle)의 타입인 std::io::Stdin의 인스턴스를 돌려준다
        */

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

    // 변수선언하기
    let foo = 5; // immutable
    let bar = 5; // mutable

    println!("foo: {}, bar: {}", foo,bar);


}