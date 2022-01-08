use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use colored::*;

fn get_colored(keyword: &str) -> ColoredString {
    match keyword {
        "CLS" | "RET" | "SYS" => keyword.cyan(),
        "JP" | "SKP" | "SKNP" => keyword.magenta(),
        "CALL" | "RND" | "DRW" => keyword.yellow(),
        "SE" | "SNE" => keyword.blue(),
        "LD" | "OR" | "AND" | "ADD" | "XOR" | "SUB" | "SHR" | "SUBN" | "SHL" => keyword.green(),
        _ => keyword.red(),
    }
}

fn main() -> io::Result<()>{
    let mut args = env::args();

    if args.len() == 1 {
        println!("Usage: c8dis <rom_file>");
        std::process::exit(0);
    }
    let filepath = args.nth(1).unwrap();
    
    println!("Filename: {:?}", filepath);
    
    let mut f = File::open(filepath)?;
    let mut buffer = [0; 4096];
    let n = f.read(&mut buffer[..])?;
    
    println!("File size: {} bytes", n);
    
    for idx in (0..n).step_by(2) {
        let first_byte = buffer[idx];
        let second_byte = buffer[idx + 1];
    
        let line = format!("{:04X}:\t{:02X} {:02X}\t", 0x200 + idx, first_byte, second_byte);
        print!("{}", line.red());
        
        let disassembly = match first_byte & 0xf0 {
            0x00 => {
                let instruction = match first_byte & 0xf {
                    0x0 => match second_byte {
                        0xe0 => "CLS",
                        0xee => "RET",
                        _ => "#INVALID",
                    },
                    _ => "SYS"
                };
                if instruction == "SYS" {
                    format!("{} {:02X}{:02X}", get_colored(instruction), first_byte & 0xf, second_byte)
                } else {
                    format!("{}", get_colored(instruction))
                }
            },
            0x10 => {
                format!("{} {:02X}{:02X}", get_colored("JP"), first_byte & 0xf, second_byte)
            },
            0x20 => { 
                format!("{} {:02X}{:02X}", get_colored("CALL"), first_byte & 0xf, second_byte)
            },
            0x30 => {
                format!("{} V{:X}, {:02X}", get_colored("SE"), first_byte & 0xf, second_byte)
            },
            0x40 => {
                format!("{} V{:X}, {:02X}", get_colored("SNE"), first_byte & 0xf, second_byte)
            },
            0x50 => {
                format!("{} V{:X}, V{:X}", get_colored("SE"), first_byte & 0xf, (second_byte & 0xf0) >> 4)
            },
            0x60 => {
                format!("{} V{:X}, {:02X}", get_colored("LD"), first_byte & 0xf, second_byte)
            },
            0x70 => { 
                format!("{} V{:X}, {:02X}", get_colored("ADD"), first_byte & 0xf, second_byte)
            },
            0x80 => {
                let instruction = match second_byte & 0xf {
                    0 => "LD",
                    1 => "OR",
                    2 => "AND",
                    3 => "XOR",
                    4 => "ADD",
                    5 => "SUB",
                    6 => "SHR",
                    7 => "SUBN",
                    0xe => "SHL",
                    _ => "#INVALID"
                };
                if instruction != "#INVALID" {
                    format!("{} V{:X}, V{:X}", get_colored(instruction), first_byte & 0xf, (second_byte & 0xf0) >> 4)
                } else {
                    format!("{}", get_colored(instruction))
                }
            },
            0x90 => {
                format!("{} V{:X}, V{:X}", get_colored("SNE"), first_byte & 0xf, (second_byte & 0xf0) >> 4)
            },
            0xa0 => {
                format!("{} I, {:02X}{:02X}", get_colored("LD"), first_byte & 0xf, second_byte)
            },
            0xb0 => {
                format!("{} V0, {:02X}{:02X}", get_colored("JP"), first_byte & 0xf, second_byte)
            },
            0xc0 => {
                format!("{} V{:X}, {:02X}", get_colored("RND"), first_byte & 0xf, second_byte)
            },
            0xd0 => {
                format!("{} V{:X}, V{:X}, {}", get_colored("DRW"), first_byte & 0xf, (second_byte) & 0xf0 >> 4, second_byte & 0xf)
            },
            0xe0 => {
                let instruction = match second_byte {
                    0x9e => "SKP",
                    0xa1 => "SKNP",
                    _ => "#INVALID"
                };
                if instruction != "#INVALID" {
                    format!("{} V{:X}", get_colored(instruction), first_byte & 0xf)
                } else {
                    format!("{}", get_colored(instruction))
                }
            },
            0xf0 => {
                match second_byte {
                    0x07 => format!("{} V{:X}, DT", get_colored("LD"), first_byte & 0xf),
                    0x0a => format!("{} V{:X}, K", get_colored("LD"), first_byte & 0xf),
                    0x15 => format!("{} DT, V{:X}", get_colored("LD"), first_byte & 0xf),
                    0x18 => format!("{} ST, V{:X}", get_colored("LD"), first_byte & 0xf),
                    0x1e => format!("{} I, V{:X}", get_colored("ADD"), first_byte & 0xf),
                    0x29 => format!("{} F, V{:X}", get_colored("LD"), first_byte & 0xf),
                    0x33 => format!("{} B, V{:X}", get_colored("LD"), first_byte & 0xf),
                    0x55 => format!("{} [I], V{:X}", get_colored("LD"), first_byte & 0xf),
                    0x65 => format!("{} V{:X}, [I]", get_colored("LD"), first_byte & 0xf),
                    _ => format!("{}", get_colored("#INVALID"))
                }
            },
            _ => format!("{}", get_colored("#INVALID"))
        };
        
        print!("{}", disassembly);
        println!();
    }

    Ok(())
}
