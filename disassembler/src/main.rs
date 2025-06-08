use std::{collections::HashMap, env, fs::File, io::Read};

fn main() {
    let opcodes: HashMap<u8, &str> = HashMap::from([(0b100010, "mov")]);
    let reg: HashMap<u8, &str> = HashMap::from([
        (0b000, "al"),
        (0b001, "cl"),
        (0b010, "dl"),
        (0b011, "bl"),
        (0b100, "ah"),
        (0b101, "ch"),
        (0b110, "dh"),
        (0b111, "bh"),
    ]);
    let wreg: HashMap<u8, &str> = HashMap::from([
        (0b000, "ax"),
        (0b001, "cx"),
        (0b010, "dx"),
        (0b011, "bx"),
        (0b100, "sp"),
        (0b101, "bp"),
        (0b110, "si"),
        (0b111, "di"),
    ]);
    let args = env::args().collect::<Vec<String>>();
    let filename = args.get(1).expect("Missing arg");
    let mut buf = vec![];
    File::open(filename)
        .expect("Missing file")
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    println!("; generated from {}", filename);
    println!("bits 16");
    for b in buf.chunks(2) {
        // println!("; {:#08b}", b[0]);
        // println!("; {:#08b}", b[1]);
        let op = opcodes.get(&(b[0] >> 2)).expect("Unsupported opcode");
        let d = b[0] & 0b10 != 0;
        let w = b[0] & 0b1 != 0;
        let map = if w { &wreg } else { &reg };
        let src = if d {
            map.get(&((b[1] & 0b111000) >> 3))
                .expect("Register not found")
        } else {
            map.get(&(b[1] & 0b111)).expect("Register not found")
        };
        let dst = if !d {
            map.get(&((b[1] & 0b111000) >> 3))
                .expect("Register not found")
        } else {
            map.get(&(b[1] & 0b111)).expect("Register not found")
        };
        println!("{op} {src}, {dst}");
    }
}

