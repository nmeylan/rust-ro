use std::mem;

pub mod script;
mod global_variable_handler;
pub mod constant;
mod shop;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct GlobalVariableEntry {
    pub name: String,
    pub value: Value,
    pub scope: GlobalVariableScope,
    pub index: Option<usize>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum GlobalVariableScope {
    CharTemporary,
    AccountTemporary,
}

#[derive(Default)]
pub struct ScriptGlobalVariableStore {
    pub variables: Vec<GlobalVariableEntry>
}

impl ScriptGlobalVariableStore {
    pub fn push(&mut self, variable: GlobalVariableEntry) {
        self.remove_global_by_name_and_scope(&variable.name, &variable.scope);
        self.variables.push(variable);
    }

    pub fn find_global_by_name_and_scope(&self, name: &String, scope: &GlobalVariableScope) -> Option<GlobalVariableEntry> {
        self.variables.iter().find(|entry| &entry.name == name && &entry.scope == scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None)).cloned()
    }

    pub fn remove_global_by_name_and_scope(&mut self, name: &String, scope: &GlobalVariableScope) {
        let position = self.variables.iter().position(|entry| &entry.name == name && &entry.scope == scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None));
        if let Some(position) = position {
            self.variables.remove(position);
        }
    }

    pub fn find_global_array_entries(&self, name: &String, scope: GlobalVariableScope) -> Vec<GlobalVariableEntry> {
        self.variables.iter().filter(|entry| &entry.name == name && entry.scope == scope
            && entry.index.is_some()).map(|e| e.clone()).collect::<Vec<GlobalVariableEntry>>()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    String(String),
    Number(i32),
}