use std::io;

fn input() -> Vec<i32>  {
let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to readline");
let mut output:Vec<i32> = Vec::with_capacity(8);
output = line.trim().split(" ").map(|x| x.parse().expect("Failed to readline as Int")).collect();
output
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
    let mut In1 = String::new();
    io::stdin().read_line(&mut In1).ok().expect("Cannot read input"); 
    let mut A = In1.trim().parse().ok().expect("Wrong data type");
    if A == 0 {
        A = -1;
    }
    else {

    }

    println!("Enter data B: ");
    let mut In2 = String::new();
    io::stdin().read_line(&mut In2).ok().expect("Cannot read input");
    let mut B = In2.trim().parse().ok().expect("Wrong data type");
    if B == 0 {
        B = -1;
    }
    else {
        
    }

    println!("Enter code data but give spcaces between each character :");
    println!("CodeA :");
    codea = input();
    println!("CodeB :");
    codeb = input();
    println!("{:?}", codea);
    println!("{:?}", codeb);

    for i in 0..8 {
        if codea[i] == 0 {
            codea[i] = -1;
        } 
        if codeb[i] == 0 {
            codeb[i] = -1;
        }
        As.push(A*codea[i]);
        bs.push(B*codeb[i]);
        cs.push(As[i]+bs[i]);
        a = a + codea[i]*cs[i];
        b = b + codeb[i]*cs[i];
    }

    println!("CodeA: {:?},\nCodeB: {:?},\nAs   : {:?},\nBs   : {:?},\nCs   : {:?}", codea, codeb, As, bs, cs);

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
