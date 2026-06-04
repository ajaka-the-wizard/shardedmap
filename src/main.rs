use shardedmap::{Builder, custommap::map::MyCustomMap};
fn main() {
    let g = Builder::new_default_lock_with_custom_map::<String, String, MyCustomMap<String, String>>(
        100,
    );
    g.insert("email".into(), "someemail@someprovider.com".into());
    println!("Retrived: {:?}", g.get(&"email".into()).unwrap())
}
