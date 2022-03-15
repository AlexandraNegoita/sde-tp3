use std::env;

fn divide(a:i32,b:i32)->Option<i32>{
    if b==0{
        return None;
    }
    else{
        return Some(a/b);
    }
}
fn rem(a:i32,b:i32)->Option<i32>{
    if b==0{
        return None;
    }
    else{
        return Some(a%b);
    }
}
fn add(a:i32,b:i32)->i32{
    return a+b;
}
fn substract(a:i32,b:i32)->i32{
    return a-b;
}
fn multiply(a:i32,b:i32)->i32{
    return a*b;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=4{
        println!("Nu ati introdus parametri corespunzatori.");
        std::process::exit(-1);
    }
    else{
        let command=&args[1];
        let a=match args[2].parse(){
            Ok(n) =>{
                n
            }
            Err(_) => {
                println!("A nu este un numar.");
                std::process::exit(-1);
            }
        };
        let b=match args[3].parse(){
            Ok(n) =>{
                n
            }
            Err(_) => {
                println!("B nu este un numar.");
                std::process::exit(-1);
            }
        };
        if command=="add"{
            println!("a + b = {}",add(a,b));
        }
        else if command=="sub"{
            println!("a - b = {}",substract(a,b));
        }
        else if command=="mul"{
            println!("a * b = {}",multiply(a,b));
        }
        else if command=="div"{
            let p=divide(a,b);
            match p{
                None => println!("Impartire la zero."),
                Some(v) => println!("a / b = {}",v),
            }
        }
        else if command=="rem"{
            let p=rem(a,b);
            match p{
                None => println!("Impartire la zero."),
                Some(v) => println!("a % b = {}",v),
            }
        }
        else {
            println!("Comanda nu este valida.");
            std::process::exit(-1);
        }
    }
    
    
}
