use crate::parser_state::ParserState;

const MAX_LEVEL: u8 = 100;

struct ParserUtil;
impl ParserUtil {
    pub fn recursion_guard(parser_state: &ParserState, level: u8, func_name: String) -> bool {
        if level > MAX_LEVEL {
            // TODO
            return false
        } else {
            return true;
        }
    }
}