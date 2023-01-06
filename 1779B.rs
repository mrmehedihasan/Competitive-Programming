#![allow(dead_code, unused_variables)]

use std::*;
enum Read{
    String(String),
    Int(i64),
    Vector(Vec<i64>),
    Error,
}

fn read(n:u8)->Read{
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error to read");
    match n {
        0 => {
            let i:i64 = x.trim().parse().expect("Not a number");
            return Read::Int(i);
        }
        1 => {
            let v:Vec<i64> = x.split(' ').flat_map(|x| x.parse::<i64>()).collect();
            return Read::Vector(v);
        }
        2 => {
            return Read::String(x);
        }
        _ => {
            return Read::Error;
        }
    }
}


fn solve(){
    let x = match read(0) {
        Read::Int(x) => {
            x
        }
        _ => {
            todo!();
        }
    };
    let d = x/2;
    if x%2==0{
        println!("YES");
        for _i in 0..d{
            print!("1 -1 ");
        }
        print!("\n");
        return;
    }else if x==1 || x ==3{   
        println!("NO");
        return;
    }else{
        // 5-> 1 -2 1 -2 1
        //7 -> 3-> 2 -3 2 -3 2 -3 2
        println!("YES");
        let k = (x-1)/2;
        for _i in 0..k{
            print!("{} {} ", k-1, -k);
        }
        println!("{}", k-1);
        return;
    }
}

fn main() {
    let mut t = match read(0) {
        Read::Int(x) => x,
        _ => todo!(),
    };
    while t>0 {
        solve();
        t-=1;
    }
    //solve();
}
