pub trait StringUtil {
    fn fill_char_array(&self, char_array: &mut [char]);
    fn camel_to_space(&self) -> String ;

}

impl StringUtil for String {
    fn fill_char_array(&self, char_array: &mut [char]) {
        for (i, c) in self.chars().enumerate() {
            if i >= char_array.len() {
                return;
            }
            char_array[i] = c;
        }
    }
    fn camel_to_space(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                result.push(' ');
            }
            result.push(c);
        }
        result
    }
}
impl StringUtil for &str {
    fn fill_char_array(&self, char_array: &mut [char]) {
        for (i, c) in self.chars().enumerate() {
            if i >= char_array.len() {
                return;
            }
            char_array[i] = c;
        }
    }
    fn camel_to_space(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                result.push(' ');
            }
            result.push(c);
        }
        result
    }
}
