pub trait StringUtil {
    fn fill_char_array(&self, char_array: &mut [char]);
}

impl StringUtil for String {
    fn fill_char_array(&self, char_array: &mut [char]) {
        for (i, c) in self.chars().enumerate() {
            char_array[i] = c;
        }
    }
}