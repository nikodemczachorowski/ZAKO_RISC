use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::memory::Memory;

pub fn read_memory_file(filename: &String) -> Memory {
    let file = File::open(filename).expect("Could not open the file");
    let reader = BufReader::new(file);
    let mut mem = Memory::new(70000);
    let mut next_addr: u32 = 0;

    let mut lines = reader.lines();
    //NOTE: Temporary solution for ESCAPEs garbage at the start of a file
    lines.next();
    lines.next();
    lines.next();
    lines.next();
    lines.next();
    for line_result in lines {
        let line = line_result.expect("Error reading the file");
        let mut bytes = line.split_whitespace();

        bytes.next();
        for word in bytes {
            mem.write(
                next_addr,
                i32::from_str_radix(word, 16)
                    .expect(&format!("Invalid hex at {}: {}", next_addr, word)),
            );
            next_addr += 4;
        }
    }

    mem
}

pub fn read_program_file(filename: &String, mem: &mut Memory) {
    let file = File::open(filename).expect("Could not open code file");
    let reader = BufReader::new(file);
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut missing_labels: Vec<(u32, String)> = Vec::new();

    let mut next_addr: u32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Error reading the file");
        let cleared = line.replace(",", "").replace("(", " ").replace(")", "");
        let parts_vec: Vec<&str> = cleared.split_whitespace().collect();
        let mut parts = parts_vec.as_slice();
        print!("{}\n", line);
        if parts[0].ends_with(":") {
            labels
                .entry(parts[0].trim_end_matches(':').to_string())
                .or_insert(next_addr);
            parts = &parts[1..];
        }

        let opcode: u8 = match parts[0].to_uppercase().as_str() {
            "ADD" => 0x01,
            "ADDI" => 0x11,
            "SUB" => 0x02,
            "SUBI" => 0x12,
            "MUL" => 0x03,
            "MULI" => 0x13,
            "DIV" => 0x04,
            "DIVI" => 0x14,
            "AND" => 0x05,
            "ANDI" => 0x15,
            "OR" => 0x06,
            "ORI" => 0x16,
            "XOR" => 0x07,
            "XORI" => 0x17,
            "LDW" => 0x18,
            "STW" => 0x19,
            "BRZ" => 0x20,
            "BRNZ" => 0x21,
            "BRGT" => 0x22,
            "BRGE" => 0x23,
            "BRLT" => 0x24,
            "BRLE" => 0x25,
            "NOP" => 0x00,
            instr => panic!("Invalid instruction: {}", instr),
        };

        let mut word = 0;
        if opcode == 0 {
        } else if opcode & 0x10 != 0 {
            let rd = parts[3]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("RD Error");
            let r1 = parts[1]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("R1 Error");
            let imm_str = parts[2].trim_start_matches("0x");
            let imm = u16::from_str_radix(imm_str, 16).expect("Imm Error");

            word |= (opcode as u32) << 26;
            word |= (rd as u32) << 21;
            word |= (r1 as u32) << 16;
            word |= imm as u32;
        } else if opcode & 0x20 != 0 {
            let r1 = parts[1]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("R Error");
            missing_labels.push((next_addr, parts[2].to_string()));

            word |= (opcode as u32) << 26;
            word |= (r1 as u32) << 16;
        } else {
            let rd = parts[3]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("RD Error");
            let r1 = parts[1]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("R1 Error");
            let r2 = parts[2]
                .trim_start_matches('R')
                .parse::<u8>()
                .expect("R2 Error");

            word |= (opcode as u32) << 26;
            word |= (rd as u32) << 21;
            word |= (r1 as u32) << 16;
            word |= (r2 as u32) << 11;
        }

        mem.write(next_addr, word);
        next_addr += 4;
    }

    for hole in missing_labels {
        let label_addr: u16 = *labels.get(&hole.1).expect("Missing lable") as u16;
        mem.write(hole.0 + 2, label_addr);
    }
}

pub fn print_mem(mem: &Memory) {
    let mut next_addr: u32 = 0;
    while next_addr < 2016 {
        let val: i32 = mem.read(next_addr);
        print!("{:08X} ", val);
        next_addr += 4;
        if next_addr % 32 == 0 {
            print!("\n");
        }
    }
}
