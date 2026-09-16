// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // this method let you gain the ownnership
    string_slice("nice weather".into()); // this method infer what type you need
    // string("nice weather".into()); // ok, too
    string(format!("Interpolation {}", "Station")); // string cat
    string_slice(&String::from("abc")[0..1]); //[0..1] take a String ref and return a string slice
    string_slice("  hello there ".trim()); // also a slice
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // return a new ownned string unlike the trim method
}
