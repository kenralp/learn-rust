pub fn entry() {
    let mutable_var = String::from("ORIGINAL");

    println!("value: {mutable_var}");

    let mutable_var2 = mutable_var;

    // println!("value: {mutable_var}");
    println!("value: {mutable_var2}");

}
