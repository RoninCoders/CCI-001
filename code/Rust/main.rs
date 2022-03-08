
fn main() {
    println!("Hello World!");
    println!("{}", gcd(14, 15));
    println!("{}", multiplier(2, 2));
    print_string_to_upper("thiago"); // Pode se usar do "to string" porque ele entende que "thiago" é do tipo &str

    let verdadeiro: bool = true;
    println!("{}", !verdadeiro);
}

fn multiplier(x: u32, y: u32) -> u32 { // u32 é para dizer que serão aceitos apenas numeros positivos
    let result: u32 = x * y;
    return result;
}

fn print_string_to_upper(string: &str) {
    println!("{}", string.to_uppercase());
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n
}
