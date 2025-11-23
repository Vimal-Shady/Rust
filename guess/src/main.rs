use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let sec=rand::thread_rng().gen_range(1,101);
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to get input");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{return;}
    };
    match guess.cmp(&sec){
        Ordering::Less=>println!("Smaller"),
        Ordering::Greater=>println!("Larger"),
        Ordering::Equal=>println!("You Win!"),
    };

}