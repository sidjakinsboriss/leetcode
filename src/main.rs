mod roman_to_integer;

fn main() {
    let res = roman_to_integer::roman_to_int("IV".into());
    println!("{}", res);
}
