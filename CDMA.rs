use std::io;

fn input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .ok()
               .expect("Cannot read input");

    let input:i32 = input.trim()
                         .parse()
                         .ok()
                         .expect("Wrong data type");
    return input;
}

fn main() {
    let mut a = 0;
    let mut b = 0;
    let mut codea:Vec<i32> = Vec::with_capacity(8);
    let mut codeb:Vec<i32> = Vec::with_capacity(8);
    let mut As:Vec<i32> = Vec::with_capacity(8);
    let mut bs:Vec<i32> = Vec::with_capacity(8);
    let mut cs:Vec<i32> = Vec::with_capacity(8);
    println!("Enter data A : ");
    let mut A:i32 = input();
    if A == 0 {
        A = -1;
    }

    println!("Enter data B: ");
    let mut B:i32 = input();
    if B == 0 {
        B = -1;
    }

    println!("Enter CodeA: ");
    for i in 0..8 {
        let mut ele:i32 = input();
        if ele == 0 {
           ele = -1 
        }
        codea.push(ele);
    }
    println!("codeA : {:?}", codea);

    println!("Enter CodeB: ");
    for i in 0..8 {
        let mut ele:i32 = input();
        if ele == 0 {
           ele = -1 
        }
        codeb.push(ele);
    }
    println!("codeB : {:?}", codeb);

    for i in 0..8 {
        As.push(A*codea[i]);
        bs.push(B*codeb[i]);
        cs.push(As[i]+bs[i]);
        a = a + codea[i]*cs[i];
        b = b + codeb[i]*cs[i];
    }
    println!("CodeA : {:?}, CodeB: {:?}, As : {:?}, Bs: {:?}, Cs: {:?}", codea, codeb, As, bs, cs);

    if a > 0 {
        println!("\n\nA : 1");
    }
    else {
        println!("\n\nA : 0");
    }

    if b > 0 {
        println!("B : 1");
    }
    else {
        println!("B : 0");
    }
}