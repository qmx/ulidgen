use ulid::Ulid;
fn main() {
    println!("{}", Ulid::new().to_string());
}
