use std::collections::HashMap;

/*
Hashmaps consume the ownership of the key, if any non-primitive datatype is used as a key
*/
fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,
    //println!("{} {}", field_name, field_value); // this will not compile because of the loss ofownership
    map.insert(String::from("Favorite color"), String::from("Blue"));
    // this is the correct way to insert a key value pair, so that we dont loose ownership
}
