use rathena_script_lang_interpreter::lang::value::Value;

#[derive(Setters, Clone, Debug)]
pub struct Script {
    #[set]
    pub id: u32,
    pub map_name: String,
    pub name: String,
    pub sprite: u16,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    #[allow(dead_code)]
    pub x_size: u16,
    #[allow(dead_code)]
    pub y_size: u16,
    pub class_name: String,
    pub class_reference: u64,
    pub constructor_args: Vec<Value>,
    #[set]
    pub instance_reference: u64,
}

impl Script {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn dir(&self) -> u16 {
        self.dir
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
