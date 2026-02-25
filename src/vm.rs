use std::ops::Not;

use serde::{Deserialize, Serialize};

use crate::memory::Memory;

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Register {
    BP,
    SP,
    IP,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
}
impl Register {
    pub fn get_index(&self) -> usize {
        match self {
            Self::BP => 0,
            Self::SP => 1,
            Self::IP => 2,
            Self::R0 => 3,
            Self::R1 => 4,
            Self::R2 => 5,
            Self::R3 => 6,
            Self::R4 => 7,
            Self::R5 => 8,
            Self::R6 => 9,
            Self::R7 => 10,
            Self::R8 => 11,
            Self::S0 => 3,
            Self::S1 => 4,
            Self::S2 => 5,
            Self::S3 => 6,
            Self::S4 => 7,
            Self::S5 => 8,
            Self::S6 => 9,
            Self::S7 => 10,
            Self::S8 => 11,
            Self::B0 => 3,
            Self::B1 => 4,
            Self::B2 => 5,
            Self::B3 => 6,
            Self::B4 => 7,
            Self::B5 => 8,
            Self::B6 => 9,
            Self::B7 => 10,
            Self::B8 => 11,
        }
    }

    pub fn get_size_bytes(&self) -> usize {
        match self {
            Self::BP => 4,
            Self::SP => 4,
            Self::IP => 4,
            Self::R0 => 4,
            Self::R1 => 4,
            Self::R2 => 4,
            Self::R3 => 4,
            Self::R4 => 4,
            Self::R5 => 4,
            Self::R6 => 4,
            Self::R7 => 4,
            Self::R8 => 4,
            Self::S0 => 2,
            Self::S1 => 2,
            Self::S2 => 2,
            Self::S3 => 2,
            Self::S4 => 2,
            Self::S5 => 2,
            Self::S6 => 2,
            Self::S7 => 2,
            Self::S8 => 2,
            Self::B0 => 1,
            Self::B1 => 1,
            Self::B2 => 1,
            Self::B3 => 1,
            Self::B4 => 1,
            Self::B5 => 1,
            Self::B6 => 1,
            Self::B7 => 1,
            Self::B8 => 1,
        }
    }

    pub fn get_modulus(&self) -> u64 {
        match self {
            Self::BP => u32::MAX as u64 + 1,
            Self::SP => u32::MAX as u64 + 1,
            Self::IP => u32::MAX as u64 + 1,
            Self::R0 => u32::MAX as u64 + 1,
            Self::R1 => u32::MAX as u64 + 1,
            Self::R2 => u32::MAX as u64 + 1,
            Self::R3 => u32::MAX as u64 + 1,
            Self::R4 => u32::MAX as u64 + 1,
            Self::R5 => u32::MAX as u64 + 1,
            Self::R6 => u32::MAX as u64 + 1,
            Self::R7 => u32::MAX as u64 + 1,
            Self::R8 => u32::MAX as u64 + 1,
            Self::S0 => u16::MAX as u64 + 1,
            Self::S1 => u16::MAX as u64 + 1,
            Self::S2 => u16::MAX as u64 + 1,
            Self::S3 => u16::MAX as u64 + 1,
            Self::S4 => u16::MAX as u64 + 1,
            Self::S5 => u16::MAX as u64 + 1,
            Self::S6 => u16::MAX as u64 + 1,
            Self::S7 => u16::MAX as u64 + 1,
            Self::S8 => u16::MAX as u64 + 1,
            Self::B0 => u8::MAX as u64 + 1,
            Self::B1 => u8::MAX as u64 + 1,
            Self::B2 => u8::MAX as u64 + 1,
            Self::B3 => u8::MAX as u64 + 1,
            Self::B4 => u8::MAX as u64 + 1,
            Self::B5 => u8::MAX as u64 + 1,
            Self::B6 => u8::MAX as u64 + 1,
            Self::B7 => u8::MAX as u64 + 1,
            Self::B8 => u8::MAX as u64 + 1,
        }
    }
    pub fn from(value: u8) -> Self {
        match value {
            0 => Self::BP,
            1 => Self::SP,
            2 => Self::IP,
            3 => Self::R0,
            4 => Self::R1,
            5 => Self::R2,
            6 => Self::R3,
            7 => Self::R4,
            8 => Self::R5,
            9 => Self::R6,
            10 => Self::R7,
            11 => Self::R8,
            12 => Self::S0,
            13 => Self::S1,
            14 => Self::S2,
            15 => Self::S3,
            16 => Self::S4,
            17 => Self::S5,
            18 => Self::S6,
            19 => Self::S7,
            20 => Self::S8,
            21 => Self::B0,
            22 => Self::B1,
            23 => Self::B2,
            24 => Self::B3,
            25 => Self::B4,
            26 => Self::B5,
            27 => Self::B6,
            28 => Self::B7,
            29 => Self::B8,
            _ => Self::R0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum InstructionKind {
    Nop,
    MoveConst,
    Move,
    LoadConstAddr,
    StoreConstAddr,
    Load,
    Store,
    LoadOffset,
    StoreOffset,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    IAdd,
    ISub,
    IMul,
    IDiv,
    IRem,
    CmpL,
    CmpG,
    CmpE,
    ICmpL,
    ICmpG,
    ICmpE,
    Branch,
    Jmp,
    Call,
    Ret,
    Push,
    Pop,
    Not,
    And,
    Or,
    Xor,
    Svc,
    Halt,
    LoadRegOffset,
    StoreRegOffset,
    Inc,
    Dec,
    Test,
    IncWord,
    DecWord,
    IncDW,
    DecDW,
}
impl InstructionKind {
    pub fn from(value: u8) -> Self {
        use InstructionKind::*;
        match value {
            0 => Nop,
            1 => MoveConst,
            2 => Move,
            3 => LoadConstAddr,
            4 => StoreConstAddr,
            5 => Load,
            6 => Store,
            7 => LoadOffset,
            8 => StoreOffset,
            9 => Add,
            10 => Sub,
            11 => Mul,
            12 => Div,
            14 => Rem,
            15 => IAdd,
            16 => ISub,
            17 => IMul,
            18 => IDiv,
            19 => IRem,
            20 => CmpL,
            21 => CmpG,
            22 => CmpE,
            23 => ICmpL,
            24 => ICmpG,
            25 => ICmpE,
            26 => Branch,
            27 => Jmp,
            28 => Call,
            29 => Ret,
            30 => Push,
            31 => Pop,
            32 => Not,
            33 => And,
            34 => Or,
            35 => Xor,
            36 => Svc,
            37 => Halt,
            38 => LoadRegOffset,
            39 => StoreRegOffset,
            40 => Inc,
            41 => Dec,
            42 => Test,
            43 => IncWord,
            44 => DecWord,
            45 => IncDW,
            46 => DecDW,
            _ => Nop,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Instruction {
    pub kind: InstructionKind,
    //dest register
    pub r0: Register,
    pub r1: Register,
    pub r2: Register,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vm {
    pub memory: Memory,
    pub register_file: [u32; 12],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            register_file: [0; _],
        }
    }

    pub fn memory_read(&self, address: u32) -> u32 {
        self.memory.memory_read(address)
    }

    pub fn memory_write(&mut self, address: u32, value: u32) {
        self.memory.memory_write(address, value)
    }

    pub fn memory_read_word(&self, address: u32) -> u16 {
        self.memory.memory_read_word(address)
    }

    pub fn memory_write_word(&mut self, address: u32, value: u16) {
        self.memory.memory_write_word(address, value)
    }

    pub fn memory_read_byte(&self, address: u32) -> u8 {
        self.memory.memory_read_byte(address)
    }

    pub fn memory_write_byte(&mut self, address: u32, value: u8) {
        self.memory.memory_write_byte(address, value)
    }

    pub fn register_get(&mut self, register: Register) -> u32 {
        let idx = register.get_index();
        let base = self.register_file[idx] as u64;
        (base % register.get_modulus()) as u32
    }

    pub fn register_store(&mut self, register: Register, value: u32) {
        let idx = register.get_index();
        let base = (value as u64 % register.get_modulus()) as u32;
        self.register_file[idx] = base;
    }

    pub fn next_instruction_data(&mut self) -> u32 {
        let ins = self.register_file[Register::IP.get_index()];
        let out = self.memory_read(ins);
        self.register_file[Register::IP.get_index()] =
            self.register_file[Register::IP.get_index()].wrapping_add(4);
        out
    }

    pub fn next_instruction(&mut self) -> Instruction {
        let value = u32::to_le_bytes(self.next_instruction_data());
        Instruction {
            kind: InstructionKind::from(value[0]),
            r0: Register::from(value[1]),
            r1: Register::from(value[2]),
            r2: Register::from(value[3]),
        }
    }

    pub fn push(&mut self, value: u32) {
        let sp = self.register_file[Register::SP.get_index()];
        self.memory_write(sp, value);
        self.register_file[Register::SP.get_index()] =
            self.register_file[Register::SP.get_index()].wrapping_add(4);
    }

    pub fn pop(&mut self) -> u32 {
        self.register_file[Register::SP.get_index()] =
            self.register_file[Register::SP.get_index()].wrapping_sub(4);
        let sp = self.register_file[Register::SP.get_index()];
        self.memory_read(sp)
    }

    pub fn run_instruction(&mut self) {
        let ins = self.next_instruction();
        match ins.kind {
            InstructionKind::Nop => {
                return;
            }
            InstructionKind::MoveConst => {
                let value = self.next_instruction_data();
                self.register_store(ins.r0, value);
            }
            InstructionKind::Move => {
                let value = self.register_get(ins.r1);
                self.register_store(ins.r0, value);
            }
            InstructionKind::LoadConstAddr => {
                let addr = self.next_instruction_data();
                let value = self.memory_read(addr);
                self.register_store(ins.r0, value);
            }
            InstructionKind::StoreConstAddr => {
                let addr = self.next_instruction_data();
                let value = (self.register_get(ins.r0) as u64 % ins.r0.get_modulus()) as u32;
                match ins.r0.get_size_bytes() {
                    4 => {
                        self.memory_write(addr, value);
                    }
                    2 => {
                        self.memory_write_word(addr, value as u16);
                    }
                    1 => {
                        self.memory_write_byte(addr, value as u8);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            InstructionKind::Load => {
                let base = self.register_get(ins.r1);
                let value = self.memory_read(base);
                self.register_store(ins.r0, value);
            }
            InstructionKind::Store => {
                let addr = self.register_get(ins.r1);
                let value = self.register_get(ins.r0);
                match ins.r1.get_size_bytes() {
                    4 => {
                        self.memory_write(addr, value);
                    }
                    2 => {
                        self.memory_write_word(addr, value as u16);
                    }
                    1 => {
                        self.memory_write_byte(addr, value as u8);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            InstructionKind::LoadOffset => {
                let address = self.register_get(ins.r1);
                let offset = self.next_instruction_data().cast_signed();
                let addr = address.wrapping_add_signed(offset);
                let value = self.memory_read(addr);
                self.register_store(ins.r0, value);
            }
            InstructionKind::StoreOffset => {
                let address = self.register_get(ins.r1);
                let offset = self.next_instruction_data().cast_signed();
                let addr = address.wrapping_add_signed(offset);
                let value = self.register_get(ins.r0);
                match ins.r1.get_size_bytes() {
                    4 => {
                        self.memory_write(addr, value);
                    }
                    2 => {
                        self.memory_write_word(addr, value as u16);
                    }
                    1 => {
                        self.memory_write_byte(addr, value as u8);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            InstructionKind::Add => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_add(r));
            }
            InstructionKind::Sub => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_sub(r));
            }
            InstructionKind::Mul => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_mul(r));
            }
            InstructionKind::Div => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_div(r));
            }
            InstructionKind::Rem => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_rem(r));
            }
            InstructionKind::IAdd => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_add_signed(r.cast_signed()));
            }
            InstructionKind::ISub => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_sub_signed(r.cast_signed()));
            }
            InstructionKind::IMul => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_mul(r));
            }
            InstructionKind::IDiv => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_div(r));
            }
            InstructionKind::IRem => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, l.wrapping_rem(r));
            }
            InstructionKind::CmpL => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, if l < r { 1 } else { 0 });
            }
            InstructionKind::CmpG => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, if l > r { 1 } else { 0 });
            }
            InstructionKind::CmpE => {
                let l = self.register_get(ins.r1);
                let r = self.register_get(ins.r2);
                self.register_store(ins.r0, if l == r { 1 } else { 0 });
            }
            InstructionKind::ICmpL => {
                let l = self.register_get(ins.r1).cast_signed();
                let r = self.register_get(ins.r2).cast_signed();
                self.register_store(ins.r0, if l < r { 1 } else { 0 });
            }
            InstructionKind::ICmpG => {
                let l = self.register_get(ins.r1).cast_signed();
                let r = self.register_get(ins.r2).cast_signed();
                self.register_store(ins.r0, if l > r { 1 } else { 0 });
            }
            InstructionKind::ICmpE => {
                let l = self.register_get(ins.r1).cast_signed();
                let r = self.register_get(ins.r2).cast_signed();
                self.register_store(ins.r0, if l == r { 1 } else { 0 });
            }
            InstructionKind::Branch => {
                let value = self.register_get(ins.r1);
                let next = self.next_instruction_data();
                if value == 1 {
                    self.register_file[Register::IP.get_index()] = next;
                }
            }
            InstructionKind::Jmp => {
                let next = self.next_instruction_data();
                self.register_file[Register::IP.get_index()] = next;
            }
            InstructionKind::Call => {
                let next = self.next_instruction_data();
                let ip = self.register_get(Register::IP);
                self.push(ip);
                self.register_store(Register::IP, next);
            }
            InstructionKind::Ret => {
                let ip = self.pop();
                self.register_store(Register::IP, ip);
            }
            InstructionKind::Push => {
                let r = self.register_get(ins.r0);
                self.push(r);
            }
            InstructionKind::Pop => {
                let v = self.pop();
                self.register_store(ins.r0, v);
            }
            InstructionKind::Not => {
                let value = self.register_get(ins.r1);
                let v = value.not();
                self.register_store(ins.r0, v);
            }
            InstructionKind::And => {
                let v1 = self.register_get(ins.r1);
                let v2 = self.register_get(ins.r2);
                let v = v1 & v2;
                self.register_store(ins.r0, v);
            }
            InstructionKind::Or => {
                let v1 = self.register_get(ins.r1);
                let v2 = self.register_get(ins.r2);
                let v = v1 | v2;
                self.register_store(ins.r0, v);
            }
            InstructionKind::Xor => {
                let v1 = self.register_get(ins.r1);
                let v2 = self.register_get(ins.r2);
                let v = v1 ^ v2;
                self.register_store(ins.r0, v);
            }
            InstructionKind::Svc => {
                todo!()
            }
            InstructionKind::Halt => todo!(),
            InstructionKind::LoadRegOffset => {
                let address = self.register_get(ins.r1);
                let offset = self.register_get(ins.r2).cast_signed();
                let addr = address.wrapping_add_signed(offset);
                let value = self.memory_read(addr);
                self.register_store(ins.r0, value);
            }
            InstructionKind::StoreRegOffset => {
                let address = self.register_get(ins.r1);
                let offset = self.register_get(ins.r2).cast_signed();
                let addr = address.wrapping_add_signed(offset);
                let value = self.register_get(ins.r0);
                match ins.r1.get_size_bytes() {
                    4 => {
                        self.memory_write(addr, value);
                    }
                    2 => {
                        self.memory_write_word(addr, value as u16);
                    }
                    1 => {
                        self.memory_write_byte(addr, value as u8);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            InstructionKind::Inc => {
                let value = self.register_get(ins.r1).wrapping_add(1);
                self.register_store(ins.r0, value);
            }
            InstructionKind::Dec => {
                let value = self.register_get(ins.r1).wrapping_sub(1);
                self.register_store(ins.r0, value);
            }
            InstructionKind::Test => {
                let value = if self.register_get(ins.r1) != 0 { 1 } else { 0 };
                self.register_store(ins.r0, value);
            }
            InstructionKind::IncWord => {
                let value = self.register_get(ins.r1).wrapping_add(2);
                self.register_store(ins.r0, value);
            }
            InstructionKind::DecWord => {
                let value = self.register_get(ins.r1).wrapping_sub(2);
                self.register_store(ins.r0, value);
            }
            InstructionKind::IncDW => {
                let value = self.register_get(ins.r1).wrapping_add(4);
                self.register_store(ins.r0, value);
            }
            InstructionKind::DecDW => {
                let value = self.register_get(ins.r1).wrapping_sub(4);
                self.register_store(ins.r0, value);
            }
        }
    }
}
