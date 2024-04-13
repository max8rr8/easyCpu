mod exec;

use clap::Parser;
use easycpu_lib::parser::ParsePosition;
use exec::DebugCpu;
use std::{fs, io::Write, process::exit};

use easycpu_lib::asm::{
    compile::compile, disasm::disassemle_instruction, err::CompileError, parse::parse_listing,
    parse::Atom,
};
use easycpu_lib::cpu::Instruction;

fn compile_file(src: std::path::PathBuf, dst: std::path::PathBuf) -> Result<(), String> {
    let contents =
        fs::read_to_string(&src).map_err(|e| format!("Failed to read file {:#?}: {}", src, e))?;

    let program = parse_listing(contents.as_str());
    let program = program.map_err(|x| format!("Failed to parse: {:#?}", x))?;

    let mut errors: Vec<(ParsePosition, CompileError)> = Vec::new();
    let mut parsed: Vec<Atom> = Vec::new();

    for line in program {
        match line.compiled {
            Ok(c) => parsed.push(c),
            Err(e) => errors.push((line.start_pos, e)),
        }
    }

    if !errors.is_empty() {
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

fn load_u16_file(src: std::path::PathBuf) -> Vec<u16> {
    let assembled = fs::read(src).unwrap();
    assembled
        .chunks(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

fn dissassemle_file(src: std::path::PathBuf) -> Result<(), String> {
    let assembled = load_u16_file(src);
    let dissassembled: Vec<String> = assembled
        .into_iter()
        .map(Instruction::decode)
        .map(disassemle_instruction)
        .collect();
    println!("{}", dissassembled.join("\n"));
    Ok(())
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "easycpu_toolkit")]
#[command(bin_name = "easycpu_toolkit")]
enum EasyCpuToolkit {
    Asm(Asm),
    Disasm(DisAsm),
    Exec(Exec),
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

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Exec {
    #[arg(index = 1)]
    initram: std::path::PathBuf,
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
        EasyCpuToolkit::Exec(args) => {
            let init_ram = load_u16_file(args.initram);
            let mut cpu = DebugCpu::new(init_ram);
            // dissassemle_file
            cpu.run();
            Ok(())
        }
    };
    if let Err(e) = res {
        eprintln!("{}", e);
        exit(1);
    }
}
