fn main() {
    /*
    변수 선언
    let 키워드를 사용하며 기본값으로 immutable 변수를 선언한다.
     */
    let name = "홍길동";
    println!("안녕하세요, {}!", name);

    // cannot assign twice to immutable variable
    // immutable 변수는 값을 변경할 수 없다. 컴파일 오류발생
    // name = "이몽룡";

    // shadowing
    // immutable 변수를 mutable 변수로 변경하는 방법(?)으로
    // 같은 이름의 변수를 선언하면서 let 키워드를 사용한다.
    // 이미 선언된 변수는 컴파일 과정에서 다른 이름으로 변경되고
    // 새롭게 선언된 변수는 이후의 코드에서 사용(read) 할 수 있다
    let name = "이몽룡";
    println!("안녕하세요, {}!", name);

    let mut nation;
    /*
    변수에 값을 할당하면 최소 한번 이상 사용(read, use) 해야 한다
    그렇지 않으면 컴파일러는 경고를 발생시킨다.
     */
    nation = "대한민국";
    println!("{} 만세", nation);

    nation = "Korea";
    println!("{} 만세", nation);

    println!("대한민국만세");

    // type 지정하여 변수선언하기
    let num: i32 = 5;
    println!("The value of num is: {}", num);

    // 문자열형 포인트 변수 선언하기
    let str: &str = "Hello, world!";
    println!("문자열 포인터 &str {}", str);

    // 문자열형 변수 선언하기
    let str: String = String::from("Hello, world!");
    println!("문자열 변수 {str}");

    // 일반 문자열을 변수에 할당하면 문자열형 포인트 변수가 되고,
    // to_string() 함수를 통하여 변환하면 문자열 변수가 된다
    let str = "우리나라 만세";
    println!("문자열 포인터 &str {}", str);

    let str = "Republic of Korea".to_string();
    println!("문자열 변수 {str}");
    
}
