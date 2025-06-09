use clap::{Parser, Subcommand, arg, command};
use phf::phf_map;
use std::{fs::File, io::{Read, Write}, slice::Iter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    bin_file: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Asm,
    AsciiBytes,
}

static OP: phf::Map<[u8; 1], fn(&u8, &mut Iter<u8>)> = phf_map! {
    [0b100010] => mov,
    [0b1011] => im_mov,
};

static REG: phf::Map<[u8; 1], &str> = phf_map! {
    [0b000] => "al",
    [0b001] => "cl",
    [0b010] => "dl",
    [0b011] => "bl",
    [0b100] => "ah",
    [0b101] => "ch",
    [0b110] => "dh",
    [0b111] => "bh",
};

static WREG: phf::Map<[u8; 1], &str> = phf_map! {
    [0b000] => "ax",
    [0b001] => "cx",
    [0b010] => "dx",
    [0b011] => "bx",
    [0b100] => "sp",
    [0b101] => "bp",
    [0b110] => "si",
    [0b111] => "di",
};

fn main() {
    let cli = Cli::parse();
    let mut buf = vec![];
    File::open(&cli.bin_file)
        .expect("Missing file")
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    match cli.command {
        Some(Commands::Asm) => asm(&buf, &cli.bin_file),
        Some(Commands::AsciiBytes) => ascii(&buf, &cli.bin_file),
        _ => asm(&buf, &cli.bin_file),
    }
}

fn ascii(buf: &[u8], file: &str) {
    println!("ASCII Binary representation of {}", file);
    for b in buf {
        println!("{:#010b} ", b);
    }
}

fn asm(buf: &[u8], file: &str) {
    println!("; generated from {}", file);
    println!("bits 16");
    let mut buf = buf.iter();
    while let Some(b) = buf.next() {
        let op = OP
            .get(&[b >> 4])
            .or_else(|| OP.get(&[b >> 2]))
            .expect("OpCode not supported");
        op(b, &mut buf);
    }
    // for b in buf.chunks(2) {
    //     let op = OP.get(&[b[0] >> 2]).expect("OpCode not supported");
    //     op(b);
    // }
}

fn mov(b: &u8, buf: &mut Iter<u8>) {
    let d = b & 0b10 != 0;
    let w = b & 0b1 != 0;
    let reg = if w { &WREG } else { &REG };

    let b = buf.next().expect("No second byte");
    let src = if !d {
        reg.get(&[((b & 0b111000) >> 3)])
            .expect("Register not found")
    } else {
        reg.get(&[(b & 0b111)]).expect("Register not found")
    };
    let dst = if d {
        reg.get(&[((b & 0b111000) >> 3)])
            .expect("Register not found")
    } else {
        reg.get(&[(b & 0b111)]).expect("Register not found")
    };
    println!("mov {dst}, {src}");
}

fn im_mov(b: &u8, buf: &mut Iter<u8>) {
    let w = b & 0b1000 != 0;
    let reg = if w { &WREG } else { &REG };
    let dst = reg.get(&[b & 0b111]).expect("Register not found");
    let data1 = buf.next().expect("No immediate data followed");
    let data2 = if w {
        buf.next().expect("w is set, but no more data is provided")
    } else {
        &0b0
    };
    let data = u16::from_le_bytes([*data1, *data2]);
    println!("mov {dst}, {data}",);
}
