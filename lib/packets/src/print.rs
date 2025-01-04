use std::fmt::Debug;
use std::slice::Iter;

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

impl PrettyOutput for [u8] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
    }
}

impl PrettyOutput for [i8] {
    fn pretty_output(&self) -> String {
        pretty_output_primitive_array(self.iter())
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
    let mut result = String::new();
    iter.for_each(|c|  {
        result.push_str(format!("{:?}", c).as_str());
    });
    format!("[{}]", result)
}