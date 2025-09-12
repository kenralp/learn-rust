struct StructType {
    data_i32: i32,
    data_f64: f64,
    data_str: String
}

impl StructType {
    pub fn new(d_i32: i32, d_f64: f64, d_str: &str) -> StructType {
        StructType {
            data_i32: d_i32,
            data_f64: d_f64,
            data_str: String::from(d_str)
        }
    }
}

pub fn entry() {

}
