// use cpu;
mod asm;
mod cpu;

use asm::{compile::compile, err::CompileError, parse::Parsed, disasm::disassemle_instruction};
use clap::Parser;
use cpu::Instruction;
use std::{fs, io::Write};

use crate::asm::parse::parse_listing;

fn compile_file(src: std::path::PathBuf, dst: std::path::PathBuf) -> Result<(), String> {
    let contents =
        fs::read_to_string(&src).map_err(|e| format!("Failed to read file {:#?}: {}", src, e))?;

    let program = parse_listing(contents.as_str());
    let program = program.map_err(|x| format!("Failed to parse: {:#?}", x))?;

    let mut errors: Vec<(usize, CompileError)> = Vec::new();
    let mut parsed: Vec<Parsed> = Vec::new();

    for line in program {
        match line.compiled {
            Ok(c) => parsed.push(c),
            Err(e) => errors.push((line.line_number, e)),
        }
    }

    if errors.len() > 0 {
        let s: Vec<String> = errors
            .iter()
            .map(|(line, err)| format!("Line {}: {:#?}", line, err))
            .collect();
        return Err(s.join("\n"));
    }

    let compiled = compile(parsed).map_err(|e| format!("Error: {:#?}", e))?;
    let bytes: Vec<_> = compiled.iter().flat_map(|x| x.to_be_bytes()).collect();
    {
        let mut file = fs::File::create(&dst)
            .map_err(|e| format!("Failed to create file {:#?}: {}", dst, e))?;
        // Write a slice of bytes to the file
        file.write_all(&bytes)
            .map_err(|e| format!("Failed to write file {:#?}: {}", dst, e))?;
    }
    Ok(())
}

fn dissassemle_file(src: std::path::PathBuf) -> Result<(), String> {
    let assembled = fs::read(&src).map_err(|e| format!("Failed to read file {:#?}: {}", src, e))?;
    let assembled = assembled
        .chunks(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]));
    let dissassembled: Vec<String> = assembled.map(|x| Instruction::decode(x)).map(|x| disassemle_instruction(x)).collect();
    println!("{}", dissassembled.join("\n"));
    Ok(())
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "easycpu_toolkit")]
#[command(bin_name = "easycpu_toolkit")]
enum EasyCpuToolkit {
    Asm(Asm),
    Disasm(DisAsm),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Asm {
    #[arg(index = 1)]
    src: std::path::PathBuf,

    #[arg(short = 'O', default_value = "./ram.bin")]
    output: std::path::PathBuf,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct DisAsm {
    #[arg(index = 1)]
    src: std::path::PathBuf,
    // TODO: add output to file with flags
    // #[arg(short = 'O', default_value = "-")]
    // output: std::path::PathBuf,
}

fn main() {
    let res: Result<(), String> = match EasyCpuToolkit::parse() {
        EasyCpuToolkit::Asm(args) => compile_file(args.src, args.output),
        EasyCpuToolkit::Disasm(args) => {
            // dissassemle_file
            dissassemle_file(args.src)
        }
    };

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
