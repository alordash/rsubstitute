struct Struct {
    number: i32,
    string: String
}

impl Struct {
    pub fn new(number: i32, string: String) -> Self {
        Self {
            number, string
        }
    }
    
    pub fn get_number(&self) -> i32 {
        self.number
    }
    
    pub fn get_string(&self) -> &str {
        &self.string
    }
    
    pub fn format(&self) -> String {
        let number = self.get_number();
        let string = self.get_string();
        let result = format!("Struct, number = {number}, string = {string}");
        return result;
    }
}

fn main() {
    println!("done")
}