use std::*;
#[allow(dead_code, unused_variables)]
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
    let n = match read(0) {
        Read::Int(x) => x,
        
        Read::Error => todo!(),
        Read::String(_) => todo!(),
        Read::Vector(_) => todo!(),
    };
    let str = match read(2) {
        Read::String(x) => {
            x
        },
        _ => {
            "None".to_string()
        }
    };

    let res = str.rfind("RL");
    
    match res{
        Some(x) => {
                println!("{}", 0);
        }
        None => {
            let res = str.rfind("LR");
            match res {
                Some(x) => {
                    println!("{}", x+1);
                }
                None => {
                    println!("-1");
                }
            }
        }

    }
}


fn main(){

    let t = match read(0) {
        Read::Int(x) => {
            x
        },
        _ => {
            0
        }
    };
    
    for i in 1..=t{
        solve();
    }
    
}
