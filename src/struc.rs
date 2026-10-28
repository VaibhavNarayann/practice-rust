// structs in rust let you structure data together. Similer to objects in javascript

struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let user1 = User {
        active: true, 
        username: String::from("someusername123"),
        email: String::from("someone@gmail.com"), 
        sign_in_count: 1, 
    }; 
    print!("User 1 username: {:?}", user1.username);
}