fn main() {
    println!("Hello, world!");
    /*
    8비트 정수	i8
    16비트 정수	i16
    32비트 정수	i32
    64비트 정수	i64
    128비트 정수	i128
    아키텍처	isize
    부호 없는 8비트 정수	u8
    부호 없는 16비트 정수	u16
    부호 없는 32비트 정수	u32
    부호 없는 64비트 정수	u64
    부호 없는 128비트 정수	u128
    부호 없는 아키텍처	usize
    불리언	bool
    문자열	String
    문자열 슬라이스	str
    32비트 부동소수점 실수	f32
    64비트 부동소수점 실수	f64
     */


     // 변수 형 변환
     let num1: f64 = 1.2;
     let num2 = num1 as i32;
     println!("type Cast : f64({num1}) -> i32({num2})");


     // 상수선언
     const MAX_POINTS: u32 = 100_000;
     println!("상수선언 : MAX_POINTS = {MAX_POINTS}");

}
