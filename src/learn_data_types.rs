struct UserDefined {
    pub data_1: i32,
    pub data_2: i32,
    pub data_3: i32
}

impl UserDefined {
    pub fn new(d1: i32, d2: i32, d3: i32) -> UserDefined {
        UserDefined {
            data_1: d1,
            data_2: d2,
            data_3: d3
        }
    }
}

pub fn entry() {
    println!("[from: learn_data_types]");
    let user_defined = UserDefined::new(100, 300, 500);

    println!("user_defined's value:\ndata_1 = {}, data_2 = {}, data_3 = {}",
        user_defined.data_1, user_defined.data_2, user_defined.data_3);
}
