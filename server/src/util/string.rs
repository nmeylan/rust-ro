pub trait StringUtil {
    fn fill_char_array(&self, char_array: &mut [char]);
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
}