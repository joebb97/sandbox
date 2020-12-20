use std::error::Error;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct Instruction<'a> {
    pub assembled_val: u32,
    pub linenum: u32,
    pub label: &'a str,
    pub opcode: &'a str,
    pub args: Vec<&'a str>,
}

impl Instruction<'_> {

    #[inline]
    fn possible_instrs() -> HashSet<String> {
        let mut set = HashSet::new();
        let items_to_insert = ["add", "nor", "lw", "sw", "beq", "jalr", "halt", "noop", ".fill"];
        for item in items_to_insert.iter() {
            set.insert(item.to_string());
        }
        set
    }

    pub fn from_line<'a>(linenum: u32, line: &'a String) ->
        Result<Box<dyn Assemble + 'a>, Box<dyn Error>> {
        let mut pieces = line.split_whitespace();
        let first_piece = pieces.next().ok_or("No first piece")?;
        let mut label = "";
        let opcode;
        if !Instruction::possible_instrs().contains(first_piece) {
            label = first_piece;
            opcode = pieces.next().ok_or("No opcode after label")?;
        } else {
            opcode = first_piece;
        }
        let the_rest: Vec<&str> = pieces.collect();
        let mut ret_instr = Instruction{
            assembled_val: 0u32,
            linenum: linenum,
            label: label,
            opcode: opcode,
            args: vec![],
        };
        let ret: Result<Box<dyn Assemble + 'a>, Box<dyn Error>> = match opcode  {
            "add"|"nor" => {
                ret_instr.args = vec![the_rest[0], the_rest[1], the_rest[2]];
                Ok(Box::new(RTypeInstruction {
                    instr: ret_instr
                }))
            },
            "lw"|"sw"|"beq" => {
                ret_instr.args = vec![the_rest[0], the_rest[1], the_rest[2]];
                Ok(Box::new(ITypeInstruction {
                    instr: ret_instr
                }))
            },
            "jalr" => {
                ret_instr.args = vec![the_rest[0], the_rest[1]];
                Ok(Box::new(JTypeInstruction {
                    instr: ret_instr
                }))
            },
            ".fill" => {
                ret_instr.args = vec![the_rest[0]];
                Ok(Box::new(FillInstruction {
                    instr: ret_instr
                }))
            },
            // Covers "halt" and "noop" cases
            "halt"|"noop" => {
                Ok(Box::new(OTypeInstruction {
                    instr: ret_instr
                }))
            }
            _ => {
                Err("Unexpected insturction opcode")?
            }

        };
        return ret;
    }

    #[inline]
    pub fn set_opcode(&mut self, contents: u32) {
        self.assembled_val |= contents << 22;
    }

    #[inline]
    pub fn set_reg_a(&mut self, contents: u32) {
        self.assembled_val |= contents << 19;
    }

    #[inline]
    pub fn set_reg_b(&mut self, contents: u32) {
        self.assembled_val |= contents << 16;
    }

    #[inline]
    pub fn set_dest(&mut self, contents: u32) {
        self.assembled_val |= contents;
    }

    pub fn set_offset(&mut self, contents: &mut i16) -> Result<(), Box<dyn Error>>{
        // Maximum and minimum values for a signed 16 bit number
        self.assembled_val |= (*contents as u16) as u32;
        return Ok(())
    }

}

pub trait Assemble {
    fn assemble(&mut self, _map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>>;
    fn get_instr(&self) -> &Instruction;
}


pub struct RTypeInstruction<'a> {
    instr: Instruction<'a>
}

impl<'a> Assemble for RTypeInstruction<'a> {
    fn assemble(&mut self, _map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>> {
        let instr = &mut self.instr;
        let opcode = match instr.opcode {
            "add" => 0b000,
            "nor" => 0b001,
            _ => Err("Unexpected RTypeInstruction opcode")?
        };
        instr.set_opcode(opcode);
        instr.set_reg_a(instr.args[0].parse()?);
        instr.set_reg_b(instr.args[1].parse()?);
        instr.set_dest(instr.args[2].parse()?);
        return Ok(instr.assembled_val);
    }

    fn get_instr(&self) -> &Instruction  {
        &self.instr
    }

}

pub struct ITypeInstruction<'a> {
    instr: Instruction<'a>
}

impl<'a> Assemble for ITypeInstruction<'a> {
    fn assemble(&mut self, map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>> {
        let instr = &mut self.instr;
        let opcode = match instr.opcode {
            "lw" => 0b010,
            "sw" => 0b011,
            "beq" => 0b100,
            _ => Err("Unexpected RTypeInstruction opcode")?
        };
        instr.set_opcode(opcode);
        instr.set_reg_a(instr.args[0].parse()?);
        instr.set_reg_b(instr.args[1].parse()?);
        let parse_val: Result<i32, _> = instr.args[2].parse();
        match parse_val {
            Ok(offset) => {
                instr.set_offset(&mut (offset as i16))?;
            }
            Err(_) => {
                let dst_label = &instr.args[2];
                let label_pos = map.get(&dst_label.to_string()).ok_or(
                    "Missing label in the label map"
                )?;
                let label_pos = *label_pos as i16;
                let mut label_pos = if instr.opcode == "beq" {
                    label_pos - ((instr.linenum + 1) as i16)
                } else {
                    label_pos
                };
                instr.set_offset(&mut label_pos)?;
            }
        };
        Ok(instr.assembled_val)
    }
    fn get_instr(&self) -> &Instruction  {
        &self.instr
    }

}

pub struct JTypeInstruction<'a> {
    instr: Instruction<'a>
}

impl<'a> Assemble for JTypeInstruction<'a> {
    fn assemble(&mut self, _map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>> {
        let instr = &mut self.instr;
        instr.set_opcode(0b101);
        instr.set_reg_a(instr.args[0].parse()?);
        instr.set_reg_b(instr.args[1].parse()?);
        return Ok(instr.assembled_val);
    }
    fn get_instr(&self) -> &Instruction  {
        &self.instr
    }

}

pub struct OTypeInstruction<'a> {
    instr: Instruction<'a>
}

impl<'a> Assemble for OTypeInstruction<'a> {
    fn assemble(&mut self, _map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>> {
        let instr = &mut self.instr;
        let opcode = match instr.opcode {
            "halt" => 0b110,
            "noop" => 0b111,
            _ => Err("Unexpected opcode in OTypeInstruction")?
        };
        instr.set_opcode(opcode);
        return Ok(instr.assembled_val);
    }
    fn get_instr(&self) -> &Instruction  {
        &self.instr
    }
}

pub struct FillInstruction<'a> {
    instr: Instruction<'a>
}

impl<'a> Assemble for FillInstruction<'a> {
    fn assemble(&mut self, map: &HashMap<String, u32>) -> Result<u32, Box<dyn Error>> {
        let parse_val: Result<i32, _> = self.instr.args[0].parse();
        let ret = match parse_val {
            Ok(offset) => Ok(offset as u32),
            Err(_) =>  {
                let dst_label = self.instr.args[0];
                let label_pos = map.get(&dst_label.to_string()).ok_or(
                    "Missing label in the label map"
                )?;
                Ok(*label_pos as u32)
            }
        };
        return ret;
    }

    fn get_instr(&self) -> &Instruction  {
        &self.instr
    }
}
