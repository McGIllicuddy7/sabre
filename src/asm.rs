use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::vm::{InstructionKind, Register};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Constant {
    Label(String),
    Value(u32),
    IValue(i32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Ir {
    Nop,
    MoveConst {
        to: Register,
        value: Constant,
    },
    Move {
        to: Register,
        from: Register,
    },
    LoadConstAddr {
        to: Register,
        from: Constant,
    },
    StoreConstAddr {
        from: Register,
        to: Constant,
    },
    Load {
        to: Register,
        from: Register,
    },
    Store {
        from: Register,
        to: Register,
    },
    LoadOffset {
        to: Register,
        from: Register,
        offset: Constant,
    },
    StoreOffset {
        from: Register,
        to: Register,
        offset: Constant,
    },
    LoadRegOffset {
        to: Register,
        from: Register,
        offset: Register,
    },
    StoreRegOffset {
        from: Register,
        to: Register,
        offset: Register,
    },

    Add {
        output: Register,
        left: Register,
        right: Register,
    },
    Sub {
        output: Register,
        left: Register,
        right: Register,
    },
    Mul {
        output: Register,
        left: Register,
        right: Register,
    },
    Div {
        output: Register,
        left: Register,
        right: Register,
    },
    Rem {
        output: Register,
        left: Register,
        right: Register,
    },
    IAdd {
        output: Register,
        left: Register,
        right: Register,
    },
    ISub {
        output: Register,
        left: Register,
        right: Register,
    },
    IMul {
        output: Register,
        left: Register,
        right: Register,
    },
    IDiv {
        output: Register,
        left: Register,
        right: Register,
    },
    IRem {
        output: Register,
        left: Register,
        right: Register,
    },
    CmpL {
        output: Register,
        left: Register,
        right: Register,
    },
    CmpE {
        output: Register,
        left: Register,
        right: Register,
    },
    CmpG {
        output: Register,
        left: Register,
        right: Register,
    },
    ICmpL {
        output: Register,
        left: Register,
        right: Register,
    },
    ICmpE {
        output: Register,
        left: Register,
        right: Register,
    },
    ICmpG {
        output: Register,
        left: Register,
        right: Register,
    },
    Branch {
        cond: Register,
        to: Constant,
    },
    Jmp {
        to: Constant,
    },
    Call {
        to: Constant,
    },
    Ret,
    Push {
        reg: Register,
    },
    Pop {
        reg: Register,
    },
    Not {
        output: Register,
        input: Register,
    },
    Inc {
        output: Register,
        input: Register,
    },
    Dec {
        output: Register,
        input: Register,
    },
    IncWord {
        output: Register,
        input: Register,
    },
    DecWord {
        output: Register,
        input: Register,
    },
    IncDw {
        output: Register,
        input: Register,
    },
    DecDw {
        output: Register,
        input: Register,
    },
    Test {
        output: Register,
        input: Register,
    },
    And {
        output: Register,
        left: Register,
        right: Register,
    },
    Or {
        output: Register,
        left: Register,
        right: Register,
    },
    Xor {
        output: Register,
        left: Register,
        right: Register,
    },
    DefineBytes {
        contents: Vec<u8>,
    },
    DefineWords {
        contents: Vec<u16>,
    },
    DefineValues {
        contents: Vec<Constant>,
    },
    Svc,
    Halt,
}
impl Ir {
    pub fn size(&self) -> usize {
        match self {
            Self::MoveConst { to: _, value: _ } => 8,
            Self::LoadConstAddr { to: _, from: _ } => 8,
            Self::StoreConstAddr { from: _, to: _ } => 8,
            Self::LoadOffset {
                to: _,
                from: _,
                offset: _,
            } => 8,
            Self::StoreOffset {
                from: _,
                to: _,
                offset: _,
            } => 8,
            Self::Branch { cond: _, to: _ } => 8,
            Self::Call { to: _ } => 8,
            Self::Jmp { to: _ } => 8,
            Self::DefineBytes { contents } => contents.len(),
            Self::DefineWords { contents } => contents.len() * 2,
            Self::DefineValues { contents } => contents.len() * 4,
            _ => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Unit {
    pub instructions: Vec<Ir>,
    pub size_bytes: usize,
    pub labels: HashMap<String, usize>,
    pub globals: HashMap<String, usize>,
}
impl Unit {
    pub fn push_instruction(&mut self, instr: Ir) {
        self.size_bytes += instr.size();
        self.instructions.push(instr);
    }
    pub fn compile_line(&mut self, line: &str) -> Result<(), String> {
        let mut values = line.split_ascii_whitespace().take_while(|x| *x != ";");
        let Some(mut base) = values.next() else {
            return Ok(());
        };
        if let Some(label) = base.strip_suffix(":") {
            self.labels.insert(label.to_string(), self.size_bytes);
            let Some(tmp) = values.next() else {
                return Ok(());
            };
            base = tmp;
        }
        match base {
            "global" => {
                let Some(next) = values.next() else {
                    return Err("expected label instead found end of line".into());
                };
                if let Some(n) = values.next() {
                    return Err(format!("expected end of line instead found {}", n));
                }
                if let Some(label) = base.strip_suffix(":") {
                    self.labels.insert(label.to_string(), self.size_bytes);
                } else {
                    return Err(format!(
                        "expected label(terminated with ':') instead found {}",
                        next
                    ));
                }
            }
            "define-bytes" => {
                let next = line.strip_prefix("define-bytes").unwrap();
                if let Some(v) = next.strip_prefix('"') {
                    if let Some(v2) = v.strip_suffix('"') {
                        let bytes = v2.as_bytes().to_vec();
                        self.push_instruction(Ir::DefineBytes { contents: bytes });
                    } else {
                        return Err(format!(
                            "expected list of u8 or string literal instead found:{}",
                            v
                        ));
                    }
                } else {
                    let mut list = Vec::new();
                    while let Some(n) = values.next() {
                        if let Ok(x) = n.parse() {
                            list.push(x);
                        } else {
                            return Err(format!("expected u8 instead found:{}", n));
                        }
                    }
                    self.push_instruction(Ir::DefineBytes { contents: list });
                }
            }
            "define-words" => {
                let mut list = Vec::new();
                while let Some(n) = values.next() {
                    if let Ok(x) = n.parse() {
                        list.push(x);
                    } else {
                        return Err(format!("expected u16 instead found:{}", n));
                    }
                }
                self.push_instruction(Ir::DefineWords { contents: list });
            }
            "define-values" => {
                let mut list = Vec::new();
                while let Some(n) = values.next() {
                    list.push(parse_constant(n));
                }
                self.push_instruction(Ir::DefineValues { contents: list });
            }
            "nop" => {
                self.push_instruction(Ir::Nop);
            }
            "mov" => {
                let Some(dest_name) = values.next() else {
                    return Err("expected register instead found end of line".into());
                };
                let Some(source_name) = values.next() else {
                    return Err("expected register instead found end of line".into());
                };
                let dest = parse_register(dest_name)?;
                if let Ok(source) = parse_register(source_name) {
                    self.push_instruction(Ir::Move {
                        to: dest,
                        from: source,
                    });
                } else {
                    let num = parse_constant(source_name);
                    self.push_instruction(Ir::MoveConst {
                        to: dest,
                        value: num,
                    });
                }
            }
            "load" | "store" => {
                let is_load = base == "load";
                let col: Vec<&str> = values.collect();
                if col.len() < 2 {
                    return Err(format!(
                        "expected 2 or 3 arguments instead found:{} arguments",
                        col.len()
                    ));
                } else if col.len() == 2 {
                    let dest = parse_register(col[0])?;
                    let ins = if let Ok(src) = parse_register(col[1]) {
                        if is_load {
                            Ir::Load {
                                to: dest,
                                from: src,
                            }
                        } else {
                            Ir::Store {
                                to: src,
                                from: dest,
                            }
                        }
                    } else {
                        let c = parse_constant(col[1]);
                        if is_load {
                            Ir::LoadConstAddr { to: dest, from: c }
                        } else {
                            Ir::StoreConstAddr { to: c, from: dest }
                        }
                    };
                    self.push_instruction(ins);
                } else if col.len() == 3 {
                    let r0 = parse_register(col[0])?;
                    let r1 = parse_register(col[1])?;
                    let ins = if let Ok(r2) = parse_register(col[2]) {
                        if is_load {
                            Ir::LoadRegOffset {
                                to: r0,
                                from: r1,
                                offset: r2,
                            }
                        } else {
                            Ir::StoreRegOffset {
                                to: r0,
                                from: r1,
                                offset: r2,
                            }
                        }
                    } else {
                        let c = parse_constant(col[2]);
                        if is_load {
                            Ir::LoadOffset {
                                to: r0,
                                from: r1,
                                offset: c,
                            }
                        } else {
                            Ir::StoreOffset {
                                to: r0,
                                from: r1,
                                offset: c,
                            }
                        }
                    };
                    self.push_instruction(ins);
                } else {
                    return Err(format!(
                        "expected 2 or 3 arguments instead found:{} arguments",
                        col.len()
                    ));
                }
            }
            "add" | "sub" | "div" | "mul" | "rem" | "iadd" | "isub" | "idiv" | "imul" | "irem"
            | "and" | "or" | "xor" | "cmp" | "icmp" => {
                let col: Vec<&str> = values.collect();
                if col.len() != 3 {
                    return Err(format!(
                        "expected 2 or 3 arguments instead found:{} arguments",
                        col.len()
                    ));
                }
                let output = parse_register(col[0])?;
                let left = parse_register(col[1])?;
                let right = parse_register(col[2])?;
                let ins = match base {
                    "add" => Ir::Add {
                        output,
                        left,
                        right,
                    },
                    "sub" => Ir::Sub {
                        output,
                        left,
                        right,
                    },
                    "div" => Ir::Div {
                        output,
                        left,
                        right,
                    },
                    "mul" => Ir::Mul {
                        output,
                        left,
                        right,
                    },
                    "rem" => Ir::Rem {
                        output,
                        left,
                        right,
                    },
                    "cmpls" => Ir::CmpL {
                        output,
                        left,
                        right,
                    },
                    "cmpeq" => Ir::CmpE {
                        output,
                        left,
                        right,
                    },
                    "cmpgr" => Ir::CmpG {
                        output,
                        left,
                        right,
                    },
                    "iadd" => Ir::IAdd {
                        output,
                        left,
                        right,
                    },
                    "isub" => Ir::ISub {
                        output,
                        left,
                        right,
                    },
                    "idiv" => Ir::IDiv {
                        output,
                        left,
                        right,
                    },
                    "imul" => Ir::IMul {
                        output,
                        left,
                        right,
                    },
                    "irem" => Ir::IRem {
                        output,
                        left,
                        right,
                    },
                    "icmpls" => Ir::ICmpL {
                        output,
                        left,
                        right,
                    },
                    "icmpeq" => Ir::ICmpE {
                        output,
                        left,
                        right,
                    },
                    "icmpgr" => Ir::ICmpG {
                        output,
                        left,
                        right,
                    },
                    "and" => Ir::And {
                        output,
                        left,
                        right,
                    },
                    "or" => Ir::Or {
                        output,
                        left,
                        right,
                    },
                    "xor" => Ir::Xor {
                        output,
                        left,
                        right,
                    },
                    _ => {
                        unreachable!()
                    }
                };
                self.push_instruction(ins);
            }
            "not" | "inc" | "dec" | "test" | "inc2" | "dec2" | "inc4" | "dec4" => {
                let col: Vec<&str> = values.collect();
                if col.len() != 2 {
                    return Err(format!("expected 2 arguments instead found:{}", col.len()));
                }
                let target = parse_register(col[0])?;
                let source = parse_register(col[1])?;
                match base {
                    "not" => {
                        self.push_instruction(Ir::Not {
                            output: target,
                            input: source,
                        });
                    }
                    "inc" => {
                        self.push_instruction(Ir::Inc {
                            output: target,
                            input: source,
                        });
                    }
                    "dec" => {
                        self.push_instruction(Ir::Dec {
                            output: target,
                            input: source,
                        });
                    }
                    "inc2" => {
                        self.push_instruction(Ir::IncWord {
                            output: target,
                            input: source,
                        });
                    }
                    "dec2" => {
                        self.push_instruction(Ir::DecWord {
                            output: target,
                            input: source,
                        });
                    }
                    "inc4" => {
                        self.push_instruction(Ir::IncWord {
                            output: target,
                            input: source,
                        });
                    }
                    "dec4" => {
                        self.push_instruction(Ir::DecWord {
                            output: target,
                            input: source,
                        });
                    }
                    "test" => {
                        self.push_instruction(Ir::Test {
                            output: target,
                            input: source,
                        });
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            "br" => {
                let col: Vec<&str> = values.collect();
                if col.len() != 2 {
                    return Err(format!("expected 2 arguments instead found:{}", col.len()));
                }
                let register = parse_register(col[0])?;
                let target = parse_constant(col[1]);
                self.push_instruction(Ir::Branch {
                    cond: register,
                    to: target,
                });
            }
            "jmp" => {
                let col: Vec<&str> = values.collect();
                if col.len() != 1 {
                    return Err(format!("expected 1 argument instead found:{}", col.len()));
                }
                let to = parse_constant(col[0]);
                self.push_instruction(Ir::Jmp { to });
            }
            "call" => {
                let col: Vec<&str> = values.collect();
                if col.len() != 1 {
                    return Err(format!("expected 1 argument instead found:{}", col.len()));
                }
                let to = parse_constant(col[0]);
                self.push_instruction(Ir::Jmp { to });
            }
            "ret" => {
                let col: Vec<&str> = values.collect();
                if col.len() > 0 {
                    return Err(format!("expected no arguments instead found:{}", col.len()));
                }
                self.push_instruction(Ir::Ret);
            }
            "halt" => {
                let col: Vec<&str> = values.collect();
                if col.len() > 0 {
                    return Err(format!("expected no arguments instead found:{}", col.len()));
                }
                self.push_instruction(Ir::Halt);
            }
            "svc" => {
                let col: Vec<&str> = values.collect();
                if col.len() > 0 {
                    return Err(format!("expected no arguments instead found:{}", col.len()));
                }
                self.push_instruction(Ir::Svc);
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn compile_file(text: &str, name: &str) -> Result<Unit, String> {
    let mut lines = text.lines();
    let mut unit = Unit {
        instructions: Vec::new(),
        size_bytes: 0,
        labels: HashMap::new(),
        globals: HashMap::new(),
    };
    let mut ln = 0;
    while let Some(n) = lines.next() {
        ln += 1;
        if let Err(e) = unit.compile_line(n) {
            return Err(format!("error file:{}, line:{}, {}", name, ln, e));
        }
    }

    for i in &mut unit.instructions {
        match i {
            Ir::MoveConst { to: _, value } => match value {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::Move { to: _, from: _ } => {}
            Ir::LoadConstAddr { to: _, from } => match from {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::StoreConstAddr { from: _, to } => match to {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::LoadOffset {
                to: _,
                from: _,
                offset,
            } => match offset {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::StoreOffset {
                from: _,
                to: _,
                offset,
            } => match offset {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::Branch { cond: _, to } => match to {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::Jmp { to } => match to {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::Call { to } => match to {
                Constant::Label(x) => {
                    if unit.labels.contains_key(x.as_str())
                        && !unit.globals.contains_key(x.as_str())
                    {
                        *x = format!("{}_{}", name, x);
                    } else {
                        *x = format!("$global_{}", x);
                    }
                }
                _ => {}
            },
            Ir::DefineValues { contents } => {
                for i in contents {
                    match i {
                        Constant::Label(x) => {
                            if unit.labels.contains_key(x.as_str())
                                && !unit.globals.contains_key(x.as_str())
                            {
                                *x = format!("{}_{}", name, x);
                            } else {
                                *x = format!("$global_{}", x);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    let mut labels = HashMap::new();
    for i in unit.labels {
        labels.insert(format!("{}_{}", name, i.0), i.1);
    }
    unit.labels = labels;
    let mut globals = HashMap::new();
    for i in unit.globals {
        globals.insert(format!("$global_{}", i.0), i.1);
    }
    unit.globals = globals;
    Ok(unit)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Binary {
    pub symbols: HashMap<String, usize>,
    pub bytes: Vec<u8>,
}
pub fn link(units: &[Unit]) -> Binary {
    let mut values: Vec<u8> = Vec::new();
    let mut globals = HashMap::new();
    let mut base = 0;
    for i in units {
        for j in i.globals.clone() {
            globals.insert(j.0, j.1 + base);
        }
        for j in i.labels.clone() {
            globals.insert(j.0, j.1 + base);
        }
        base += i.size_bytes;
    }
    for i in units {
        for j in &i.instructions {
            match j {
                Ir::Nop => {
                    values.push(InstructionKind::Nop as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                }
                Ir::MoveConst { to, value } => {
                    values.push(InstructionKind::MoveConst as u8);
                    values.push(*to as u8);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, value, &globals);
                }
                Ir::Move { to, from } => {
                    values.push(InstructionKind::Move as u8);
                    values.push(*to as u8);
                    values.push(*from as u8);
                    values.push(0);
                }
                Ir::LoadConstAddr { to, from } => {
                    values.push(InstructionKind::LoadConstAddr as u8);
                    values.push(*to as u8);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, from, &globals);
                }
                Ir::StoreConstAddr { from, to } => {
                    values.push(InstructionKind::StoreConstAddr as u8);
                    values.push(*from as u8);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, to, &globals);
                }
                Ir::Load { to, from } => {
                    values.push(InstructionKind::Load as u8);
                    values.push(*to as u8);
                    values.push(*from as u8);
                    values.push(0);
                }
                Ir::Store { from, to } => {
                    values.push(InstructionKind::Store as u8);
                    values.push(*from as u8);
                    values.push(*to as u8);
                    values.push(0);
                }
                Ir::LoadOffset { to, from, offset } => {
                    values.push(InstructionKind::LoadOffset as u8);
                    values.push(*to as u8);
                    values.push(*from as u8);
                    values.push(0);
                    store_constant(&mut values, offset, &globals);
                }
                Ir::StoreOffset { from, to, offset } => {
                    values.push(InstructionKind::StoreOffset as u8);
                    values.push(*from as u8);
                    values.push(*to as u8);
                    values.push(0);
                    store_constant(&mut values, offset, &globals);
                }
                Ir::LoadRegOffset { to, from, offset } => {
                    values.push(InstructionKind::LoadRegOffset as u8);
                    values.push(*to as u8);
                    values.push(*from as u8);
                    values.push(*offset as u8);
                }
                Ir::StoreRegOffset { from, to, offset } => {
                    values.push(InstructionKind::StoreRegOffset as u8);
                    values.push(*from as u8);
                    values.push(*to as u8);
                    values.push(*offset as u8);
                }
                Ir::Add {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Add as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Sub {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Sub as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Mul {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Mul as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Div {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Div as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Rem {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Rem as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::IAdd {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::IAdd as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::ISub {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::ISub as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::IMul {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::IMul as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::IDiv {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::IDiv as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::IRem {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::IRem as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::CmpL {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::CmpL as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::CmpE {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::CmpE as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::CmpG {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::CmpG as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::ICmpL {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::ICmpL as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::ICmpE {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::ICmpE as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::ICmpG {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::ICmpL as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Branch { cond, to } => {
                    values.push(InstructionKind::Branch as u8);
                    values.push(*cond as u8);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, to, &globals);
                }
                Ir::Jmp { to } => {
                    values.push(InstructionKind::Jmp as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, to, &globals);
                }
                Ir::Call { to } => {
                    values.push(InstructionKind::Call as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                    store_constant(&mut values, to, &globals);
                }
                Ir::Ret => {
                    values.push(InstructionKind::Ret as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                }
                Ir::Push { reg } => {
                    values.push(InstructionKind::Push as u8);
                    values.push(*reg as u8);
                    values.push(0);
                    values.push(0);
                }
                Ir::Pop { reg } => {
                    values.push(InstructionKind::Pop as u8);
                    values.push(*reg as u8);
                    values.push(0);
                    values.push(0);
                }
                Ir::Not { output, input } => {
                    values.push(InstructionKind::Not as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::And {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::And as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Or {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Or as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::Xor {
                    output,
                    left,
                    right,
                } => {
                    values.push(InstructionKind::Xor as u8);
                    values.push(*output as u8);
                    values.push(*left as u8);
                    values.push(*right as u8);
                }
                Ir::DefineBytes { contents } => {
                    for i in contents {
                        values.push(*i);
                    }
                }
                Ir::DefineWords { contents } => {
                    for i in contents {
                        let v = u16::to_le_bytes(*i);
                        values.push(v[0]);
                        values.push(v[1]);
                    }
                }
                Ir::DefineValues { contents } => {
                    for i in contents {
                        store_constant(&mut values, i, &globals);
                    }
                }
                Ir::Svc => {
                    values.push(InstructionKind::Svc as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                }
                Ir::Halt => {
                    values.push(InstructionKind::Halt as u8);
                    values.push(0);
                    values.push(0);
                    values.push(0);
                }
                Ir::Inc { output, input } => {
                    values.push(InstructionKind::Inc as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::Dec { output, input } => {
                    values.push(InstructionKind::Dec as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::IncWord { output, input } => {
                    values.push(InstructionKind::IncWord as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::DecWord { output, input } => {
                    values.push(InstructionKind::DecWord as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::IncDw { output, input } => {
                    values.push(InstructionKind::IncDW as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::DecDw { output, input } => {
                    values.push(InstructionKind::DecDW as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
                Ir::Test { output, input } => {
                    values.push(InstructionKind::Test as u8);
                    values.push(*output as u8);
                    values.push(*input as u8);
                    values.push(0);
                }
            }
        }
    }
    Binary {
        symbols: globals,
        bytes: values,
    }
}

pub fn parse_register(name: &str) -> Result<Register, String> {
    match name {
        "bp" => Ok(Register::BP),
        "sp" => Ok(Register::SP),
        "ip" => Ok(Register::IP),

        "r0" => Ok(Register::R0),
        "r1" => Ok(Register::R1),
        "r2" => Ok(Register::R2),
        "r3" => Ok(Register::R3),
        "r4" => Ok(Register::R4),
        "r5" => Ok(Register::R1),
        "r6" => Ok(Register::R2),
        "r7" => Ok(Register::R3),
        "r8" => Ok(Register::R4),

        "s0" => Ok(Register::S0),
        "s1" => Ok(Register::S1),
        "s2" => Ok(Register::S2),
        "s3" => Ok(Register::S3),
        "s4" => Ok(Register::S4),
        "s5" => Ok(Register::S5),
        "s6" => Ok(Register::S6),
        "s7" => Ok(Register::S7),
        "s8" => Ok(Register::S8),

        "b0" => Ok(Register::B0),
        "b1" => Ok(Register::B1),
        "b2" => Ok(Register::B2),
        "b3" => Ok(Register::B3),
        "b4" => Ok(Register::B4),
        "b5" => Ok(Register::B5),
        "b6" => Ok(Register::B6),
        "b7" => Ok(Register::B7),
        "b8" => Ok(Register::B8),

        _ => Err(format!("expected register name, instead found, {}", name)),
    }
}

pub fn parse_constant(value: &str) -> Constant {
    if let Ok(num) = value.parse::<u32>() {
        Constant::Value(num)
    } else if let Ok(num) = value.parse::<i32>() {
        Constant::Value(num.cast_unsigned())
    } else {
        Constant::Label(value.to_string())
    }
}

pub fn store_constant(values: &mut Vec<u8>, value: &Constant, globals: &HashMap<String, usize>) {
    match value {
        Constant::Label(x) => {
            if !globals.contains_key(&*x) {
                todo!();
            }
            let val = globals[&*x] as u32;
            let v = u32::to_le_bytes(val);
            values.push(v[0]);
            values.push(v[1]);
            values.push(v[2]);
            values.push(v[3]);
        }
        Constant::Value(x) => {
            let v = u32::to_le_bytes(*x);
            values.push(v[0]);
            values.push(v[1]);
            values.push(v[2]);
            values.push(v[3]);
        }
        Constant::IValue(x) => {
            let v = i32::to_le_bytes(*x);
            values.push(v[0]);
            values.push(v[1]);
            values.push(v[2]);
            values.push(v[3]);
        }
    }
}
