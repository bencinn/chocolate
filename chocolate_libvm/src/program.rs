use crate::{instruction::Instruction, virt::VMData};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Program {
    #[wasm_bindgen(skip)]
    pub build_info: Option<String>,

    #[wasm_bindgen(skip)]
    pub insts: Vec<Instruction>,
}

#[wasm_bindgen]
impl Program {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Program {
        Program {
            build_info: None,
            insts: Vec::new(),
        }
    }
    pub fn push_instruction(&mut self, inst: &str) {
        self.insts.push(Instruction::from_json(inst).unwrap());
    }

    pub fn insert_at(&mut self, inst: &str, after: u8) {
        self.insts
            .insert(after.into(), Instruction::from_json(inst).unwrap());
    }

    pub fn get_inst_len(&self) -> usize {
        self.insts.len()
    }

    pub fn run_with_new_vm(&self) {
        let mut vm = VMData::new();
        while vm.pc < self.insts.len() {
            vm.step_execute(&self.insts[vm.pc]);
        }
    }

    pub fn get_instruction_str(&self, pos: usize) -> String {
        if pos >= self.insts.len() {
            "".to_string()
        } else {
            #[cfg(target_arch = "wasm32")]
            return format!(
                "<b>{}</b> {:?}, {:?}",
                self.insts[pos].inst, self.insts[pos].param_1, self.insts[pos].param_2
            )
            .to_string();

            #[cfg(not(target_arch = "wasm32"))]
            return format!(
                "{} {:?} {:?}",
                self.insts[pos].inst, self.insts[pos].param_1, self.insts[pos].param_2
            )
            .to_string();
        }
    }

    pub fn to_raw_beautified(&self) -> String {
        let mut raw = String::new();
        for inst in &self.insts {
            let raw_inst = inst.to_raw();
            raw.push_str(&format!(
                "{:02x} {:02x} {:02x} {:02x} ",
                raw_inst[0], raw_inst[1], raw_inst[2], raw_inst[3]
            ));
        }
        raw
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        for inst in &self.insts {
            let raw_inst = inst.to_raw();
            raw.push(raw_inst[0]);
            raw.push(raw_inst[1]);
            raw.push(raw_inst[2]);
            raw.push(raw_inst[3]);
        }
        raw
    }

    pub fn from_raw(raw: &[u8]) -> Program {
        let mut program = Program::new();
        program.insts = Instruction::from_raw(&raw);
        program
    }
}
