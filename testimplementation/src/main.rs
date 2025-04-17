use libqbase::schema::*;
fn main() {
    println!("Hello, world!");
    let schema = Schema::default_schema();
    println!("{:#?}", schema);
}
