use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    instruction::{Instruction, EP},
    program::Program,
};

pub enum Status {
    Normal,
    Int(i8),
    Undefined,
}

#[wasm_bindgen]
pub struct VMData {
    // register
    reg_8bit: [i8; 16],

    // counter/pointer (should be in the reg_8bit)
    pub pc: usize,   // for instruction,
    pub next: usize, // for jump
    pub sc: usize,   // for stack

    // i8 stack, currently capped at 128
    stack: [i8; 128],

    flags: [bool; 16],

    status: Status,
}

#[wasm_bindgen]
impl VMData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let reg_8bit = [0; 16];
        let stack = [0; 128];
        VMData {
            reg_8bit,
            pc: 0,
            next: 1,
            sc: 0,
            stack,
            flags: [false; 16],
            status: Status::Normal,
        }
    }

    /// execute and retusn whether you can continue to execute
    pub fn execute_from_program(&mut self, p: &Program) -> bool {
        if self.pc >= p.insts.len() {
            false
        } else {
            self.step_execute(&p.insts[self.pc]);
            self.pc < p.insts.len()
        }
    }

    pub fn read_reg(&self, reg: i8) -> i8 {
        self.reg_8bit[(reg as usize).rem_euclid(16)]
    }

    pub fn read_stack(&self) -> Option<i8> {
        if self.sc < 1 {
            None
        } else {
            Some(self.stack[self.sc - 1])
        }
    }
}

impl VMData {
    fn write_reg(&mut self, reg: i8, val: i8) {
        self.reg_8bit[(reg as usize).rem_euclid(16)] = val;
    }
    pub fn step_execute(&mut self, inst: &Instruction) {
        let ep = Instruction::get_instruction(inst.inst, 0);
        self.next = self.pc + 1;
        match ep {
            EP::Halt => {}
            EP::Mov => {
                if let (Some(val), Some(val2)) = (inst.param_1, inst.param_2) {
                    self.write_reg(val, self.read_reg(val2));
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::AddR => {
                if let (Some(val), Some(val2)) = (inst.param_1, inst.param_2) {
                    // overflow flag
                    let first_read = self.read_reg(val);
                    //let result = first_read.checked_add(self.read_reg(val2));
                    let result = first_read.overflowing_add(self.read_reg(val2));
                    self.flags[3] = result.1;
                    self.write_reg(val, result.0);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::SubR => {
                if let (Some(val), Some(val2)) = (inst.param_1, inst.param_2) {
                    // overflow flag
                    let first_read = self.read_reg(val);
                    //let result = first_read.checked_add(self.read_reg(val2));
                    let result = first_read.overflowing_sub(self.read_reg(val2));
                    self.flags[3] = result.1;
                    self.write_reg(val, result.0);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Add => {
                if let (Some(val), Some(val2)) = (inst.param_1, inst.param_2) {
                    // overflow flag
                    let first_read = self.read_reg(val);
                    let result = first_read.overflowing_add(val2);
                    self.flags[3] = result.1;
                    self.write_reg(val, result.0);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Sub => {
                if let (Some(val), Some(val2)) = (inst.param_1, inst.param_2) {
                    // overflow flag
                    let first_read = self.read_reg(val);
                    let result = first_read.overflowing_sub(val2);
                    self.flags[3] = result.1;
                    self.write_reg(val, result.0);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Push => {
                if let Some(val) = inst.param_1 {
                    self.push_stack(val);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Pop => {
                if let Some(val) = inst.param_1 {
                    let val2 = self.pop_stack();
                    if let Some(val2) = val2 {
                        self.write_reg(val, val2);
                    } else {
                        self.status = Status::Undefined;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Read => {
                if let Some(val) = inst.param_1 {
                    if let Some(val2) = self.read_stack() {
                        self.write_reg(val, val2);
                    } else {
                        self.status = Status::Undefined;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::PushR => {
                if let Some(val) = inst.param_1 {
                    self.push_stack(self.read_reg(val));
                }
            }
            EP::Interrupt => {
                if let Some(val) = inst.param_1 {
                    self.status = Status::Int(val);
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Cmp => {
                if let (Some(dest), Some(source)) = (inst.param_1, inst.param_2) {
                    self.flags[0] = self.read_reg(source) == self.read_reg(dest);
                    self.flags[1] = self.read_reg(source) < self.read_reg(dest);
                    self.flags[2] = self.read_reg(source) > self.read_reg(dest);
                    self.flags[4] = self.read_reg(source) == 0;
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Jmp => {
                if let Some(val) = inst.param_1 {
                    self.next = val as usize;
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Je => {
                if let Some(val) = inst.param_1 {
                    if self.flags[0] {
                        self.next = val as usize;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Jz => {
                if let Some(val) = inst.param_1 {
                    if self.flags[4] {
                        self.next = val as usize;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Jrmp => {
                if let Some(val) = inst.param_1 {
                    self.next = self.read_reg(val) as usize;
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Jre => {
                if let Some(val) = inst.param_1 {
                    if self.flags[0] {
                        self.next = self.read_reg(val) as usize;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }
            EP::Jrz => {
                if let Some(val) = inst.param_1 {
                    if self.flags[4] {
                        self.next = self.read_reg(val) as usize;
                    }
                } else {
                    self.status = Status::Undefined;
                }
            }

            EP::Invalid => {
                self.status = Status::Undefined;
            }
        }
        self.pc = self.next;
    }

    fn push_stack(&mut self, val: i8) {
        self.stack[self.sc] = val;
        self.sc += 1;
    }
    fn pop_stack(&mut self) -> Option<i8> {
        if self.sc > 0 {
            self.sc -= 1;
            Some(self.stack[self.sc])
        } else {
            None
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_vm_init() {
        let vm = VMData::new();
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.sc, 0);
    }

    #[test]
    fn test_vm_stack() {
        let mut vm = VMData::new();
        assert_eq!(vm.read_stack(), None);
        vm.push_stack(12);
        assert_eq!(vm.read_stack(), Some(12));
        assert_eq!(vm.pop_stack(), Some(12));
        assert_eq!(vm.read_stack(), None);
    }

    #[test]
    fn test_execute() {
        let mut vm = VMData::new();
        let inst1 = Instruction {
            inst: 0,
            param_1: None,
            param_2: None,
        };
        let inst2 = Instruction {
            inst: 0,
            param_1: None,
            param_2: None,
        };
        assert_eq!(vm.pc, 0);
        vm.step_execute(&inst1);
        assert_eq!(vm.pc, 1);
        vm.step_execute(&inst2);
        assert_eq!(vm.pc, 2);
    }

    #[test]
    fn test_stackpush_inst() {
        let mut vm = VMData::new();
        let inst1 = Instruction {
            inst: 7,
            param_1: Some(6),
            param_2: None,
        };
        vm.step_execute(&inst1);
        assert_eq!(vm.read_stack(), Some(6));
        assert_eq!(vm.sc, 1);
    }

    #[test]
    fn test_iset_register() {
        let mut vm = VMData::new();
        vm.reg_8bit[3] = 69;
        let inst_mov = Instruction {
            inst: 1,
            param_1: Some(2),
            param_2: Some(3),
        };
        let inst_add = Instruction {
            inst: 4,
            param_1: Some(2),
            param_2: Some(5),
        };
        let inst_subr = Instruction {
            inst: 3,
            param_1: Some(2),
            param_2: Some(3),
        };
        vm.step_execute(&inst_mov);
        assert_eq!(vm.reg_8bit[2], 69);
        vm.step_execute(&inst_add);
        assert_eq!(vm.reg_8bit[2], 74);
        vm.step_execute(&inst_subr);
        assert_eq!(vm.reg_8bit[2], 5);
    }

    #[test]
    fn test_status() {
        // mov without parameter
        let invalid_instruction = Instruction {
            inst: 1,
            param_1: None,
            param_2: None,
        };
        let int = Instruction {
            inst: 10,
            param_1: Some(0),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.step_execute(&invalid_instruction);
        assert!(matches!(vm.status, Status::Undefined));
        vm.step_execute(&int);
        assert!(matches!(vm.status, Status::Int(0)));
    }

    #[test]
    fn test_int_stuck() {
        let int = Instruction {
            inst: 10,
            param_1: Some(0),
            param_2: None,
        };
        let halt = Instruction {
            inst: 0,
            param_1: None,
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.step_execute(&int);
        assert!(matches!(vm.status, Status::Int(0)));
        vm.step_execute(&halt);
        // since there are no external interrupt resolves, the status should be the same unless there is any errors.
        assert!(matches!(vm.status, Status::Int(0)));
    }

    #[test]
    fn test_overflow_signal() {
        let add = Instruction {
            inst: 4,
            param_1: Some(0),
            param_2: Some(127),
        };
        let sub = Instruction {
            inst: 5,
            param_1: Some(0),
            param_2: Some(127),
        };
        let mut vm = VMData::new();
        vm.reg_8bit[0] = 127;
        vm.step_execute(&add);
        assert!(vm.flags[3]);
        vm.flags[3] = false;
        vm.step_execute(&sub);
        assert!(vm.flags[3]);
    }

    #[test]
    fn test_compare() {
        let cmp = Instruction {
            inst: 11,
            param_1: Some(0),
            param_2: Some(1),
        };
        let mut vm = VMData::new();
        vm.reg_8bit[0] = 1;
        vm.reg_8bit[1] = 1;
        vm.step_execute(&cmp);
        assert!(vm.flags[0]);
        assert!(!vm.flags[1]);
        assert!(!vm.flags[2]);
    }

    #[test]
    fn test_jump() {
        let jmp = Instruction {
            inst: 12,
            param_1: Some(3),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.step_execute(&jmp);
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn test_jump_reg() {
        let jmp = Instruction {
            inst: 15,
            param_1: Some(1),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.reg_8bit[1] = 3;
        vm.step_execute(&jmp);
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn test_read_empty_stack() {
        let read = Instruction {
            inst: 8,
            param_1: Some(0),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.step_execute(&read);
        assert!(matches!(vm.status, Status::Undefined));
    }

    #[test]
    fn test_cmp_source_zero() {
        let cmp = Instruction {
            inst: 11,
            param_1: Some(1),
            param_2: Some(0),
        };
        let mut vm = VMData::new();
        vm.reg_8bit[0] = 0;
        vm.reg_8bit[1] = 1;
        vm.step_execute(&cmp);
        assert!(vm.flags[4]);
        let cmp2 = Instruction {
            inst: 11,
            param_1: Some(1),
            param_2: Some(1),
        };
        vm.step_execute(&cmp2);
        assert!(!vm.flags[4]);
    }

    #[test]
    fn test_cmp() {
        let cmp = Instruction {
            inst: 11,
            param_1: Some(1),
            param_2: Some(0),
        };
        let mut vm = VMData::new();
        vm.reg_8bit[0] = 1;
        vm.reg_8bit[1] = 1;
        vm.step_execute(&cmp);
        assert!(vm.flags[0]);
        assert!(!vm.flags[1]);
        assert!(!vm.flags[2]);
        let cmp_lt = Instruction {
            inst: 11,
            param_1: Some(2),
            param_2: Some(1),
        };
        vm.reg_8bit[2] = 2;
        vm.step_execute(&cmp_lt);
        assert!(!vm.flags[0]);
        assert!(vm.flags[1]);
        assert!(!vm.flags[2]);
    }

    #[test]
    fn test_jz() {
        let jz = Instruction {
            inst: 14,
            param_1: Some(3),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.flags[4] = true;
        vm.step_execute(&jz);
        assert_eq!(vm.pc, 3);
        vm.flags[4] = false;
        vm.step_execute(&jz);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_push_from_reg() {
        let add = Instruction {
            inst: 4,
            param_1: Some(1),
            param_2: Some(12),
        };
        let push_r = Instruction {
            inst: 9,
            param_1: Some(1),
            param_2: None,
        };
        let mut vm = VMData::new();
        vm.step_execute(&add);
        vm.step_execute(&push_r);
        assert_eq!(vm.read_stack(), Some(12));
    }
}
