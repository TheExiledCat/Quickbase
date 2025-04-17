use libqbase::schema::*;
fn main() {
    let schema = Schema::default_schema();
    println!("{:#?}", schema);
    schema.export();
}
