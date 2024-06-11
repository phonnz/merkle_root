use sha256::{digest, try_digest};

fn concatenate_and_hash(s1: &str, s2: &str) -> String {
    let concatenated = format!("{}{}", s1, s2);
    digest(concatenated)
}

fn main() {
    let string1 = "77d519a56a3bb197bca02ed25f880a122487914556d587588e633c8368d13053";
    let string2 = "915961583d426ff5d6726ee59ff7e1ad234d8343f60c57ab023b21741fdba723";

    let hash_value = concatenate_and_hash(string1, string2);

    println!("The hash of the concatenated strings is: {}", hash_value);
}
