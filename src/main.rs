use shardedmap::{Builder, custommap::map::MyCustomMap};
fn main() {
    // let d = shardedmap::Builder::new_default_lock_with_hashmap::<String, String>(300);
    // let e = shardedmap::Builder::new_default_lock_with_btreemap::<String, String>(100);
    // d.confirm();
    // e.confirm();

    let g = Builder::new_default_lock_with_custom_map::<String,String,MyCustomMap<String,String>>(100);

}
