use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main(){
    let sec=rand::thread_rng().gen_range(1,101);
    loop{
    println!("Enter you Guess: ");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to get input");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{return;}
    };
    match guess.cmp(&sec){
        Ordering::Less=>println!("{}","Smaller".red()),
        Ordering::Greater=>println!("{}","Larger".red()),
        Ordering::Equal=>{println!("{}","You Win!".green());break;},
    };
}

}