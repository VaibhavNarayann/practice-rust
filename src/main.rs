/*
This code fails because the ownership of str is moved to the 
get_length function. Once the function's scop ends, the str variable is no longer vaild.
*/

fn main() {
    let str = String::from("Vaibhav");
    let len = get_length(str);
    println!("{}", len);

    print!("{}", str);
}

fn get_length(str: String) -> usize {
    return str.len();
}



// Fix #1 - Transferring back ownership


 pub fn fix(){
    let str = String::from("Vaibhav");
    let (str, len) = get_length(str);
    println!("{}", len); 

    print!("{}", str); 
 }

 pub fn get_length(str:String) -> (String, usize) {
    let len = str.len(); 
    return (str, len); 
 }

 // Fix #2 - References/Borrowing 