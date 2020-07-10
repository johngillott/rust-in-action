#[derive(PartialEq)]
struct Hostname(String);

// if we implement the 'From' trait then the 'To' trait is satisfied
impl From<String> for Hostname {
    fn from(item: String) -> Self {
        Hostname(item)
    }
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    // was:
    // if host == ordinary_string
    if host == ordinary_string.into() {
        // println!("huh?");
        println!("aaah.")
    };
}
