use std::slice::Iter;
use std::fmt::Debug;

pub trait PrettyOutput {
    fn pretty_output(&self) -> String;
}

impl PrettyOutput for [char] {
    fn pretty_output(&self) -> String {
        let mut result = String::new();
        self.iter().for_each(|c|  {
            if *c != '\0' {
                result.push(*c);
            }
        });
        result
    }
}

impl PrettyOutput for [i16] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
    }
}

impl PrettyOutput for [u16] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
    }
}

impl PrettyOutput for [i32] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
    }
}

impl PrettyOutput for [u32] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
    }
}

impl PrettyOutput for Vec<u8> {
    fn pretty_output(&self) -> String {
        format!("{:?}", self)
    }
}

fn pretty_output_primitive_array<T : Debug>(iter: Iter<T>) -> String {
    let result = String::new();
    iter.for_each(|c|  {
        format!("{}, {:?}", result, c);
    });
    format!("[{}]", result)
}