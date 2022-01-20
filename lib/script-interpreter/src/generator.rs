use bnf::{Grammar, Production, Term};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use regex::{Captures, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref AFTER_UNDERSCORE_CHAR_REGEX: Regex = Regex::new(r"_(\w)").unwrap();
    static ref UPPERCASE_CHAR_REGEX: Regex = Regex::new(r"([A-Z])").unwrap();
    static ref FIRST_CHAR_REGEX: Regex = Regex::new(r"^(\w)").unwrap();
}

trait TermType {
    fn is_terminal(&self) -> bool;
    fn is_non_terminal(&self) -> bool;
}

impl TermType for Term {
    fn is_terminal(&self) -> bool {
        !self.is_non_terminal()
    }

    fn is_non_terminal(&self) -> bool {
        self.to_string().contains("<")
    }
}

pub fn generate() {
    let input = fs::read_to_string("lib/script-interpreter/src/grammar.bnf").unwrap();
    let grammar: Grammar = input.parse().unwrap();
    ast_generate(&grammar);
    parser_generate(&grammar);
}

/*
* Parser generator
 */
pub fn parser_generate(grammar: &Grammar) {
    let output_path = Path::new("lib/script-interpreter/src");
    let mut parser_file = File::create(output_path.join("parser.rs")).unwrap();

    write_file_header(&mut parser_file);
    parser_file.write_all(b"use crate::token::Token;\n").unwrap();
    parser_file.write_all(b"use crate::ast::expression::*;\n").unwrap();
    parser_file.write_all(b"use crate::parser_state::ParserState;\n\n").unwrap();

    parser_file.write_all(b"pub fn parse(tokens: &Vec<Token>) -> Box<dyn Expression> {\n").unwrap();
    parser_file.write_all(b"    let mut parser_state = ParserState::new(tokens);\n").unwrap();
    parser_file.write_all(b"}\n").unwrap();

    parser_file.write_all(b"pub fn parse_token(parser_state: &mut ParserState, token_type: TokenType) -> bool {\n").unwrap();
    parser_file.write_all(b"    parser_state.consume(token_type)\n").unwrap();
    parser_file.write_all(b"}\n\n").unwrap();

    for production in grammar.productions_iter() {
        let expression_terms = expression_terms(production);
        parser_generate_method_comment(&mut parser_file, production);
        parser_file.write_all(format!("pub fn parse_{}(parser_state: &mut ParserState) -> bool {{\n", to_snake_case(&production.lhs.to_string())).as_bytes()).unwrap();
        parser_file.write_all(b"    let mut result = false;\n").unwrap();
        for (identifier, terms) in expression_terms.iter() {
            parser_file.write_all(b"    if !result {\n").unwrap();
            for (i, term) in terms.iter().enumerate() {
                let mut prefix = "";
                if i > 0 {
                    prefix = "result && "
                }
                if term.1 == "Token" {
                    parser_file.write_all(format!("        result = {}parse_token(parser_state, TokenType::{});\n", prefix, to_camel_case(&term.0)).as_bytes()).unwrap();
                } else {
                    parser_file.write_all(format!("        result = {}parse_{}(parser_state);\n", prefix, term.0).as_bytes()).unwrap();
                }
            }
            parser_file.write_all(b"    }\n").unwrap();
        }
        parser_file.write_all(b"    result\n").unwrap();
        parser_file.write_all(b"}\n").unwrap();
    }

}

fn parser_generate_method_comment(parser_file: &mut File, production: &Production) {
    parser_file.write_all(
        format!("// {}\n",
                production.rhs_iter().map(|expression|
                    format!("{}", expression.terms_iter().map(|term| term.to_string()).collect::<Vec<String>>().join(" "))
                ).collect::<Vec<String>>().join(" | "))
            .as_bytes()).unwrap();
}

/*
* AST generator
*/
pub fn ast_generate(grammar: &Grammar) {
    let output_path = Path::new("lib/script-interpreter/src/ast");

    let mut mod_file = File::create(output_path.join("mod.rs")).unwrap();
    let mut visitor_file = File::create(output_path.join("visitor.rs")).unwrap();

    write_file_header(&mut mod_file);
    write_file_header(&mut visitor_file);

    mod_file.write_all(b"pub mod expression;\n").unwrap();
    mod_file.write_all(b"pub mod visitor;\n").unwrap();
    visitor_file.write_all(b"pub trait Visitor {\n").unwrap();
    ast_generate_expression_trait(output_path);

    for production in grammar.productions_iter() {
        let name = production.lhs.to_string();
        ast_generate_expression(output_path, production);
        mod_file.write_all(format!("pub mod {};\n", to_snake_case(&name)).as_bytes());
        visitor_file.write_all(format!("  fn visit_{}(&self, expression: &dyn crate::ast::expression::Expression);\n", to_snake_case(&name)).as_bytes()).unwrap();
    }
    visitor_file.write_all(b"}\n");
}

fn ast_generate_expression_trait(output_path: &Path) {
    let mut file = File::create(output_path.join("expression.rs")).unwrap();
    write_file_header(&mut file);
    file.write_all(b"pub trait Expression {\n").unwrap();
    file.write_all(b"    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>);\n").unwrap();
    file.write_all(b"}\n").unwrap();
}

fn ast_generate_expression(output_path: &Path, production: &Production) {
    let name = production.lhs.to_string();
    let file_name = format!("{}.rs", to_snake_case(&name));
    let mut file = File::create(output_path.join(file_name)).unwrap();
    write_file_header(&mut file);
    let expression_terms = expression_terms(production);
    ast_generate_expression_enum_def(&mut file, production, &expression_terms);
    ast_generate_expression_enum_impl(&mut file, production, &expression_terms);
    ast_generate_expression_trait_impl(&mut file, production);
}

fn ast_generate_expression_enum_def(file: &mut File, production: &Production, expression_terms: &Vec<(String, Vec<(String, String)>)>) {
    let name = production.lhs.to_string();
    file.write_all(format!("pub enum {} {{\n", to_camel_case(&name)).as_bytes()).unwrap();
    for entry in expression_terms.iter() {
        let term_name = &entry.0;
        let term = &entry.1;
        if term.len() > 1 {
            file.write_all(format!("    {}{{ {} }},\n", term_name, term.iter()
                .map(|t| {
                    format!("{}: {}", to_snake_case(&t.0), type_name(&t.1))
                })
                .collect::<Vec<String>>().join(", ")).as_bytes()).unwrap();
        } else {
            let term_type = &term[0].1;
            file.write_all(format!("    {}({}),\n", term_name, type_name(term_type)).as_bytes()).unwrap();
        }
    }
    file.write_all(format!("    \n").as_bytes()).unwrap();
    file.write_all(b"}\n\n").unwrap();
}

fn ast_generate_expression_enum_impl(file: &mut File, production: &Production, expression_terms: &Vec<(String, Vec<(String, String)>)>) {
    let name = production.lhs.to_string();
    file.write_all(format!("impl {} {{\n", to_camel_case(&name)).as_bytes()).unwrap();
    for entry in expression_terms.iter() {
        let term_name = &entry.0;
        let term = &entry.1;
        file.write_all(format!("    pub fn build_from_{}({}) -> Self {{\n", to_rust_field_name(term_name),
                               term.iter().map(|t| format!("{}: {}", to_snake_case(&t.0), type_name(&t.1)))
                                   .collect::<Vec<String>>().join(", ")).as_bytes()).unwrap();
        if term.len() > 1 {
            file.write_all(format!("        {}::{}{{ {} }}\n", to_camel_case(&name), term_name,
                                   term.iter().map(|t| to_snake_case(&t.0))
                                       .collect::<Vec<String>>().join(", ")).as_bytes()).unwrap();
        } else {
            file.write_all(format!("        {}::{}({})\n", to_camel_case(&name), term_name, to_rust_field_name(term_name)).as_bytes());
        }
        file.write_all(b"    }\n").unwrap();
    }
    file.write_all(b"}\n\n").unwrap();
}

fn ast_generate_expression_trait_impl(file: &mut File, production: &Production) {
    let name = production.lhs.to_string();
    file.write_all(format!("impl crate::ast::expression::Expression for {} {{\n", to_camel_case(&name)).as_bytes()).unwrap();
    file.write_all(b"    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {\n").unwrap();
    file.write_all(format!("        visitor.visit_{}(self)\n", to_snake_case(&name)).as_bytes()).unwrap();
    file.write_all(b"    }\n\n").unwrap();
    file.write_all(b"}\n\n").unwrap();
}

/*
* Common
 */

fn module_name(term_type: &String) -> String {
    let mut module: String;
    if term_type == "Token" {
        module = "crate::token::".to_string();
    } else {
        module = format!("crate::ast::{}::", to_rust_field_name(term_type));
    }
    module
}

fn type_name(term_type: &String) -> String {
    if term_type == "Token" {
        return format!("{}{}", module_name(term_type), term_type);
    }
    format!("Box<{}{}>", module_name(term_type), term_type)
}

fn expression_terms(production: &Production) -> Vec<(String, Vec<(String, String)>)> {
    let name = production.lhs.to_string();
    let mut expression_terms: Vec<Vec<(String, String)>> = vec![];
    for expression in production.rhs_iter() {
        let mut has_terminal = false;
        let mut terms = Vec::<(String, String)>::new();
        for term in expression.terms_iter() {
            if term.is_terminal() {
                has_terminal = true;
                terms.push((to_rust_field_name(&term.to_string()), "Token".to_string()));
            } else {
                terms.push((to_snake_case(&term.to_string()), to_camel_case(&term.to_string())));
            }
        }
        expression_terms.push(terms);
    }
    let mut expression_terms_with_name = Vec::<(String, Vec<(String, String)>)>::new();
    let mut i: u8 = 0;
    for term in expression_terms.iter() {
        if term.len() > 1 {
            expression_terms_with_name.push((format!("{}{}", to_camel_case(&name), i), term.clone()));
        } else {
            expression_terms_with_name.push((to_camel_case(&term[0].0), term.clone()));
        }
        i += 1;
    }
    expression_terms_with_name
}

fn write_file_header(file: &mut File) {
    file.write_all(b"// Generated by lib/script-interpreter/generator\n").unwrap();
    file.write_all(b"// Auto generated file do not edit manually\n\n").unwrap();
}

fn to_snake_case(name: &String) -> String {
    let mut rust_name = name.clone();
    rust_name = rust_name.replace("<", "");
    rust_name = rust_name.replace(">", "");
    rust_name
}

fn to_camel_case(bnf_name: &String) -> String {
    let mut rust_name = to_snake_case(bnf_name);
    rust_name = FIRST_CHAR_REGEX.replace(&rust_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_uppercase() }).to_string();
    rust_name = AFTER_UNDERSCORE_CHAR_REGEX.replace_all(&rust_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_uppercase() }).to_string();
    rust_name
}

fn to_rust_field_name(camel_case: &String) -> String {
    let mut field_name = camel_case.clone();
    field_name = field_name.replace("\"", "");
    field_name = FIRST_CHAR_REGEX.replace(&field_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_lowercase() }).to_string();
    field_name = AFTER_UNDERSCORE_CHAR_REGEX.replace_all(&field_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_lowercase() }).to_string();
    field_name = UPPERCASE_CHAR_REGEX.replace_all(&field_name, |caps: &Captures| { format!("_{}", caps.get(1).unwrap().as_str().to_lowercase()) }).to_string();
    field_name
}