use std::io;

fn encrypt(text: &str, shift: i32) -> String {
    let code_a = 'A' as i32;
    let code_z = 'Z' as i32;
    //A와 Z의 문자코드를 int형태로 반환
    let mut result = String::new(); //결과를 대입할 변수
    //한글자씩 변환
    for ch in text.chars() {
        let mut code = ch as i32; //문자코드로 변환
        if code_a <= code && code <= code_z{ //A와 Z 사이의 값인가
            code = (code - code_a + shift + 26) % 26 + code_a;
        } //Shift만큼 뒤로 변환
        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    println!("Let's encrypt the sentence."); //문장을 입력받기
    let mut change_sentence = String::new();
    io::stdin().read_line(&mut change_sentence).expect("Failed to read line");
    
    let enc = encrypt(&change_sentence, 3); //함수 호출
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}
