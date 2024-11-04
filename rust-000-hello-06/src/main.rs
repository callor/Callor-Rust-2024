fn main() {
    let result = plus_one(5);
    println!("Hello, world! {result}");
}


fn plus_one(x: i32) -> i32 {
    // 세미콜론이 있는 마지막 표현식은 반환값이 아니다.
    // x + 1;
    
    // 이때는 return 키워드를 사용해야 한다.
    // return x + 1; 
    


    // 세미콜론이 없는 마지막 표현식은 반환값이 된다.
    x + 1

}