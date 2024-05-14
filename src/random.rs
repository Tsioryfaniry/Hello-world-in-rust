use rand::Rng;
pub fn random_method(){
    let secret_number = rand::thread_rng().gen_range(1, 101);
println!("The secret number is: {}", secret_number);
}