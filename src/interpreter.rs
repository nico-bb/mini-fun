#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    Push,
    Pop,
    Const,
    And,
    Or,
    Nand,
}

#[derive(Clone, Copy)]
pub enum Value {
    Void,
    Bit(BitValue),
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BitValue {
    Off = 0,
    On = 1,
}

impl BitValue {
    pub fn is_true(self: BitValue) -> bool {
        return self == BitValue::On;
    }

    pub fn is_false(self: BitValue) -> bool {
        return self == BitValue::Off;
    }
}

pub struct Chunk {
    bytecode: Vec<OpCode>,
    ip: usize,
    stack: [Value; 255],
    stack_count: usize,
    const_pool: Vec<Value>,
}

const STACK_SIZE: usize = 255;

impl Chunk {
    pub fn new() -> Self {
        return Self {
            bytecode: Vec::new(),
            ip: 0,
            stack: [Value::Void; 255],
            stack_count: 0,
            const_pool: Vec::new(),
        };
    }

    pub fn push_op_multiples(&mut self, ops: &[OpCode]) {
        for op in ops {
            self.push_op(*op);
        }
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.bytecode.push(op);
    }

    pub fn reset_ip(&mut self) {
        self.ip = 0;
    }

    pub fn clear_stack(&mut self) {
        self.stack_count = 0;
    }

    pub fn execute(&mut self) -> Option<Value> {
        while self.ip < self.bytecode.len() {
            let op = self.pop_instr();

            match op {
                OpCode::Pop => {
                    self.pop_stack_value();
                }
                OpCode::Push => {
                    self.push_stack_value(Value::Void);
                }
                OpCode::Const => {
                    let const_addr = self.pop_addr();
                    let const_val = self.const_pool[const_addr];
                    self.push_stack_value(const_val);
                }
                OpCode::And => {
                    let right = self.pop_stack_value();
                    let left = self.pop_stack_value();

                    if let Value::Bit(r) = right {
                        if let Value::Bit(l) = left {
                            let on = l.is_true() && r.is_true();
                            let val = if on { BitValue::On } else { BitValue::Off };
                            self.push_stack_value(Value::Bit(val));
                        }
                    }
                }
                OpCode::Or => {
                    let right = self.pop_stack_value();
                    let left = self.pop_stack_value();

                    if let Value::Bit(r) = right {
                        if let Value::Bit(l) = left {
                            let on = l.is_true() || r.is_true();
                            let val = if on { BitValue::On } else { BitValue::Off };
                            self.push_stack_value(Value::Bit(val));
                        }
                    }
                }
                OpCode::Nand => {
                    let right = self.pop_stack_value();
                    let left = self.pop_stack_value();

                    if let Value::Bit(r) = right {
                        if let Value::Bit(l) = left {
                            let on = !(l.is_true() && r.is_true());
                            let val = if on { BitValue::On } else { BitValue::Off };
                            self.push_stack_value(Value::Bit(val));
                        }
                    }
                }
            }
        }

        let mut result: Option<Value> = None;
        if self.stack_count > 0 {
            result = self.stack.first().copied();
        }
        return result;
    }

    fn pop_instr(&mut self) -> OpCode {
        self.ip += 1;
        return self.bytecode[self.ip - 1];
    }

    fn pop_addr(&mut self) -> usize {
        return 0;
    }

    pub fn pop_stack_value(&mut self) -> Value {
        let val = self.stack[self.stack_count - 1];
        self.stack_count -= 1;
        return val;
    }

    pub fn push_stack_value(&mut self, value: Value) {
        self.stack[self.stack_count] = value;
        self.stack_count += 1;
    }
}
