use crate::{
    enums::lua_type::{LuaType, LUA_TYPE_MAP},
    util::{read_stream::ReadStream, write_stream::WriteStream},
};

#[derive(Debug)]
pub struct Constant {
    pub lua_type: LuaType,
    pub string_data: String,
    pub number_data: f64,
    pub bool_data: bool,
}

impl Constant {
    pub fn new(memory_stream: &mut ReadStream) -> Self {
        let mut new_self = Self {
            lua_type: LUA_TYPE_MAP[memory_stream.read_int8() as usize],
            string_data: String::from(""),
            number_data: 0.0,
            bool_data: false,
        };

        match new_self.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN => new_self.bool_data = memory_stream.read_int8() == 1,
            LuaType::INVALID => unreachable!(),
            LuaType::NUMBER => new_self.number_data = memory_stream.read_double(),
            LuaType::STRING => new_self.string_data = memory_stream.read_string(),
        }

        new_self
    }

    pub fn serialize(&self, write_stream: &mut WriteStream) {
        write_stream.write_int8(
            LUA_TYPE_MAP
                .iter()
                .position(|v| *v == self.lua_type)
                .expect("Failed to get lua type index") as u8,
        );

        match self.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN => write_stream.write_int8(if self.bool_data { 1 } else { 0 }),
            LuaType::INVALID => unreachable!(),
            LuaType::NUMBER => write_stream.write_double(self.number_data),
            LuaType::STRING => write_stream.write_string(&self.string_data),
        }
    }
}
