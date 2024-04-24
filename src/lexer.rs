use std::rc::Rc;
use std::ops::*;

pub enum TypeName {
    Int(u8),
    Uint(u8),
    Float(bool),
    String,
    Char,
    Bool,
}

pub enum Value {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    String(String),
    Char(char),
    Bool(bool),
}

pub struct Expression {
    pub bytes: Rc<[u8]>,
    pub typename: TypeName,
}

// convert_sign!
macro_rules! cs {
    ($item:expr,$typ:ty) => {
        $item as $typ - 128
    }
}

impl Expression {
    pub unsafe fn parse(&self) -> Value {
        match self.typename {
            TypeName::Int(byte_count) => {
                match byte_count {
                    8 => Value::I8(cs!(self.bytes[0], i8)),
                    16 => Value::I16(cs!(self.bytes[0], i16) + cs!(self.bytes[1], i16) * 128),
                    32 => Value::I32(cs!(self.bytes[0], i32) + cs!(self.bytes[1], i32) * 128
                                    + cs!(self.bytes[2], i32) * 32_768 + cs!(self.bytes[3], i32) * 8_388_608),
                    64 => Value::I64(cs!(self.bytes[0], i64) + cs!(self.bytes[1], i64) * 128
                                     + cs!(self.bytes[2], i64) * 32_768 + cs!(self.bytes[3], i64) * 8_388_608
                                     + cs!(self.bytes[4], i64) * 2_147_483_648 + cs!(self.bytes[5], i64) * 549_755_813_888
                                     + cs!(self.bytes[6], i64) * 140_737_488_355_328 + cs!(self.bytes[7], i64) * 36_028_797_018_963_968),
                }
            }
            TypeName::Char => {
                Value::Char(self.bytes[0].into())
            }
        }
    }
}

impl Add for Expression {
    type Output = Expression;
    
    fn add(self, expr: Expression) -> Self::Output {
        unsafe {
            self.parse()
        }
    }
}

pub enum Operation {
    Add(Expression, Expression)
}
