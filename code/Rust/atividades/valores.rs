fn main() {
    
    let mut a: u32 = 10;
    let mut b: u32 = 20;

    let auxiliar = a;
    a = b;
    b = auxiliar;

    println!("A:{}, B:{}", a, b);

}