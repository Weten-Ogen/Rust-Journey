struct Color(i32,i32,i32);
struct User {
    active:bool,
    sign_in_count:u64,
    username:String,
    email:String,
}
struct Point(i32,i32,i32);
fn main() {
    let black: Color = Color(0,0,0);
    let origin:Point =  Point(0,0,0);
    let email:String = String::from("Marcuoware");
    let username: String= String::from("marcuoware@gmail.com");
    let user1:User = build_user(email, username);
    let user2 :User = User { active: user1.active, sign_in_count: user1.sign_in_count, username: user1.username, email:user1.email};

    println!("{}",user2.email)
}

fn build_user(email:String,username:String) -> User {
    User{
        
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}