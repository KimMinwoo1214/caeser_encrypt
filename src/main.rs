use std::io;

fn encrypt(text: &str, shift: i32) -> String {
    let code_a = 'A' as i32;
    let code_z = 'Z' as i32;
    let mut result = String::new();
    for ch in text.chars() {
        let mut code = ch as i32;
        if code_a <= code && code <= code_z{
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    println!("Let's encrypt the sentence.");
    let mut change_sentence = String::new();
    io::stdin().read_line(&mut change_sentence).expect("Failed to read line");
    let enc = encrypt(&change_sentence, 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}