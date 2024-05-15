fn main(){
    let reference_to_nothng = dangle();
    println!("{}", reference_to_nothng);
}
fn dangle()-> String{
    let s = String::from("Hello");
    s
}