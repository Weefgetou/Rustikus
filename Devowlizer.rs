fn main() {
    let name = format!("Ignatz Trochowaszczki");
    let (name_ohne_vokale, originalname) = vokale_aus_string_exterminieren(name);
    
    println!("{}", originalname);

    println!("aus Ihrem Namen wird {} Schönen Tag noch Herr {}",name_ohne_vokale, originalname)
}

fn vokale_aus_string_exterminieren(ausgangsname: String) -> (String, String) {
    let mut rest: String = String::new();
    for c in ausgangsname.chars() {
        match c {
             'a'  | 'e' | 'i' | 'o' | 'u' => {},
             _ => {
                 rest.push(c)
            }
        }
    }
    (rest, ausgangsname)
}