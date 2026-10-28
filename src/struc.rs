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

/*
Implementing structs 

You can also implement structs, which means you can attach functions to instances of structs

(very similer to classes in TS)

*/

struct Rect {
    width: u32, 
    height: u32,
}

impl Rect {
    fn area(&self)-> u32{
        self.width * self.height
    }
}


fn main(){
    let rect = Rect {
        width: 30,
        height: 50       
    };
    print!("The area of the rectangle is {}", rect.area())
}



