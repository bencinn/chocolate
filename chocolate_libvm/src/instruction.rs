/// Instruction & Program serialization
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub inst: u8,
    pub param_1: Option<i8>,
    pub param_2: Option<i8>,
}

#[derive(Debug)]
pub enum EP {
    Halt,
    Mov,
    AddR,
    SubR,
    Add,
    Sub,
    Read,
    Push,
    Pop,
    Interrupt,
    Cmp,
    Jmp,
    Je,
    Invalid,
}

impl Instruction {
    pub fn from_json<'a>(json_str: &'a str) -> Option<Instruction> {
        let p: Option<Instruction> = serde_json::from_str(json_str).ok();
        p
    }

    pub fn get_instruction(inst: u8, version: i8) -> EP {
        assert!(version == 0);
        match inst {
            0 => EP::Halt,
            1 => EP::Mov,
            2 => EP::AddR,
            3 => EP::SubR,
            4 => EP::Add,
            5 => EP::Sub,
            6 => EP::Read,
            7 => EP::Push,
            8 => EP::Pop,
            9 => EP::Interrupt,
            10 => EP::Cmp,
            11 => EP::Jmp,
            12 => EP::Je,
            _ => EP::Invalid,
        }
    }

    fn resolve_argument_count(ep: EP) -> usize {
        match ep {
            EP::Invalid | EP::Halt => 0,
            EP::Read | EP::Push | EP::Pop | EP::Interrupt | EP::Jmp | EP::Je => 1,
            EP::Mov | EP::AddR | EP::SubR | EP::Add | EP::Sub | EP::Cmp => 2
        }
    }

    pub fn from_raw(code: &[u8]) -> Vec<Self> {
        // Raw instruction are like this.
        //        23      15       7      0
        // 00000000000000000000000000000000
        // 0-7: inst
        // 8-15: p1
        // 16-23: p2
        // 24-31: reserved
        let mut insts = Vec::new();
        let mut p = 0;
        while p<code.len() {
            let inst = code[p];
            let param_1 = Some(i8::from_ne_bytes([code[p+1]]));
            let param_2 = Some(i8::from_ne_bytes([code[p+2]]));
            let ep = Instruction::get_instruction(inst, 0);
            let count = Instruction::resolve_argument_count(ep);
            insts.push(match count {
                0 => Instruction {
                    inst,
                    param_1: None,
                    param_2: None
                },
                1 => Instruction {
                    inst,
                    param_1,
                    param_2: None
                },
                2 => Instruction {
                    inst,
                    param_1,
                    param_2
                },
                _ => unreachable!()
            });

            p+=4; // 4*8==32
        }
        insts
    }

    pub fn to_raw(&self) -> [u8; 4] {
        let mut raw = [0; 4];
        raw[0] = self.inst;
        if let Some(p1) = self.param_1 {
            raw[1] = p1 as u8;
        }
        if let Some(p2) = self.param_2 {
            raw[2] = p2 as u8;
        }
        raw
    }
}

mod tests {
    use super::*;

    #[test]
    fn instruction_parsing() {
        let str = r#"
        {
            "inst": 12,
            "param_1": 8
        }
        "#;
        let res = Instruction::from_json(str).unwrap();
        assert_eq!(res.inst, 12);
        assert_eq!(res.param_1, Some(8));
        assert_eq!(res.param_2, None);
    }

    #[test]
    fn instruction_from_raw_single() {
        let raw_instruction: [u8; 4] = [1, 2, 3, 0];
        let instructions = Instruction::from_raw(&raw_instruction);
        assert_eq!(instructions.len(), 1);
        let inst = &instructions[0];
        assert_eq!(inst.inst, 1);
        assert_eq!(inst.param_1, Some(2));
        assert_eq!(inst.param_2, Some(3));
    }

    #[test]
    fn instruction_from_raw_multi() {
        let raw_instruction: [u8; 8] = [1, 2, 3, 0, 1, 2, 5, 0];
        let instructions = Instruction::from_raw(&raw_instruction);
        assert_eq!(instructions.len(), 2);
        let inst = &instructions[0];
        assert_eq!(inst.inst, 1);
        assert_eq!(inst.param_1, Some(2));
        assert_eq!(inst.param_2, Some(3));
        let inst2 = &instructions[1];
        assert_eq!(inst2.inst, 1);
        assert_eq!(inst2.param_1, Some(2));
        assert_eq!(inst2.param_2, Some(5));
    }

    #[test]
    fn test_from_raw_to_raw() {
        let raw_instruction: [u8; 8] = [1, 2, 3, 0, 1, 2, 5, 0];
        let instructions = Instruction::from_raw(&raw_instruction);
        let raw = instructions[0].to_raw();
        let raw2 = instructions[1].to_raw();
        let joined = [raw, raw2].concat();
        assert_eq!(joined, raw_instruction);
    }
}
