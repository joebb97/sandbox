use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
mod instrs;
use instrs::{Instruction, Assemble};
use std::collections::HashMap;

pub struct Config {
    pub in_file: String,
    pub out_file: String,
    pub debug_mode: bool,
}

impl Config {

    pub fn usage() -> &'static str {
        "USAGE: assembler INPUT_FILE OUTPUF_FILE [-d|--debug]"
    }

    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3  {
            return Err(Config::usage());
        }
        let debug_mode = args.len() == 4 && 
                         ((args[3] == "-d") | (args[3] == "--debug"));

        Ok(Config {
            in_file: args[1].clone(),
            out_file: args[2].clone(),
            debug_mode
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(config.in_file)?);
    let lines : Vec<_> = reader.lines().collect();
    let mut instrs : Vec<Box<dyn Assemble>> = Vec::new();
    instrs.reserve(lines.len());
    for (linenum, line) in lines.iter().enumerate() {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                continue;
            }
            let instr = Instruction::from_line(linenum as u32, &line)?;
            instrs.push(instr);
        }
    }
    let mut map: HashMap<String, u32> = HashMap::new();
    // Make label map
    for instr in &instrs  {
        let cur_instr = instr.get_instr();
        if !cur_instr.label.is_empty() {
            map.insert(cur_instr.label.to_string(), cur_instr.linenum);
        }
    }
    for instr in &mut instrs {
        match instr.get_instr().opcode {
            ".fill" =>  {
                println!("{}", instr.assemble(&map)? as i32);
                continue
            },
            _ => println!("{}", instr.assemble(&map)?)
        }
    }

    Ok(())
}
