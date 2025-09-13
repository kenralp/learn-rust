struct StructType {
    data_i32: i32,
    data_f64: f64,
    data_str: String
}

impl StructType {
    pub fn new(d_i32: i32, d_f64: f64, d_str: &str) -> Self {
        println!("constructor StructType::new() called!");
        Self {
            data_i32: d_i32,
            data_f64: d_f64,
            data_str: String::from(d_str)
        }
    }

    pub fn show(&self) {
        println!("Data of {}: {} and {}", self.data_str, self.data_i32, self.data_f64);
    }
}

impl Drop for StructType {
    fn drop(&mut self) {
        println!("destructor StructType::drop() called!");
    }
}

pub fn entry() {
    let data = StructType::new(10, 77.8888, "Kenralp");
    data.show();
}
