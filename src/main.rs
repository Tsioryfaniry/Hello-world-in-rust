fn main(){
    let mut s = String::from("Hello world");
        let word = first_word(&s);

        println!("Slice type {}", word);
        
        s.clear();
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();//change string an array
     for (i, &item) in bytes.iter().enumerate(){//iter() methode iterateur
        if item == b' '{
            return &s[0..i];
        }
     }
     &s[..]

}