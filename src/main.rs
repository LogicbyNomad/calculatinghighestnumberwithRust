


fn main() {
    let b = highest(4,2,8);
    
    println!("{} is highest",b);
    loop_to_10();
    array_loop();
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
fn loop_to_10(){
    for n in 0..10 {
        println!("Hello {}", n);
        }
    }

fn array_loop(){
    let v = vec![4,7,9,10];

    for n in v {
        println!("{}", n);
    }
    

}