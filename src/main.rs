


fn main() {
    let b = highest(4,2,8);
    
    println!("{} is highest",b);
    loopto10();
}

fn highest(a:i32,b : u32, c : i8)->i32{
    let mut res = a;
    if b as i32 > res {
        res = b as i32;
    }
    if c as i32 > res {
        res = c as i32;
    }
    return res;
}
fn loopto10(){
    let mut n = 0;
    loop {
        n += 1;
        println!("Hello");
        if n >= 10{
            return;
        }
    }
}

