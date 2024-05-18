fn main(){
    let user1 = User{
        username: String::from("Tsiory"),
        email: String::from("tsiory@gmail.com"),
        _sign_in_count: 1,
        _active: true
    };
    let user2 = User{
        email: String::from("faniry@gmail.com"),
        username: String::from("faniry"),
      ..user1
    };
    let name = user1.email;
    let user_name2 =user1.username;
    user2.show_user(); 
    User::build_user(name,user_name2);
}
struct User{
    email:String,
    username:String,
    _sign_in_count: u16,
    _active: bool
}
impl User{
    fn show_user(&self){
        println!("Voici mon email: {} et mon user name: {}", self.email, self.username);

    }
    fn build_user(email:String, username:String) -> User{
        User { username, email, _sign_in_count: 1, _active: false }
    }
}