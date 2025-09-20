use std::{fs::{read, OpenOptions}, io::{Read,Write}};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use mc_classic;

pub fn read_4k (input: &str, mode: u8, flag: bool) -> Result<(), mc_classic::ClassicError> {

    //Reading in stream as bytes
    let stream: Vec<u8> = read(input).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    if mode != 2 {
        let mut d_stream = GzDecoder::new(&stream[..]);
        d_stream.read_to_end(&mut bytes).unwrap();
    } else {
        bytes = stream.clone();
    }

    //Creating 13a level
    let mut level: mc_classic::Level = mc_classic::Level::new();

    let mut blocks: Vec<u8> = Vec::new();

    //Converting block ids & changing ints to bytes
    for i in 0..bytes.len() {
        //println!("{}",i+1%4);
        if mode == 2 || (i+1)%4 == 0 {
            //println!("This is the current byte: {}", bytes[i]);
            if mode == 0 {
                match bytes[i] {
                    0 => blocks.push(0), //Air
                    _ => blocks.push(1) //Stone
                }
            } else {
                match bytes[i] {
                    0 => blocks.push(0), //Air
                    1 => blocks.push(2), //Grass
                    2 => blocks.push(3), //Dirt
                    3 => blocks.push(3), //Dirt
                    4 => blocks.push(1), //Stone
                    5 => if flag {blocks.push(45)} else {blocks.push(0)}, //Bricks
                    6 => blocks.push(3), //Dirt
                    7 => blocks.push(17), //Logs
                    8 => blocks.push(18), //Leaves
                    9 => if mode != 2 {blocks.push(3) /*Dirt*/} else {blocks.push(9 /*Water*/)}, 
                    10 => blocks.push(3), //Dirt
                    11 => blocks.push(3), //Dirt
                    12 => blocks.push(3), //Dirt
                    13 => blocks.push(3), //Dirt
                    14 => blocks.push(3), //Dirt
                    15 => blocks.push(3), //Dirt
                    _ => blocks.push(0)
                }
            }
        }
    }

    let mut blocks1: Vec<u8> = vec![0; 64*64*64];

    for x in 0..64{
        for y in 0..64{
            for z in 0..64{
                blocks1[x + (z * 64) + (y * 64 * 64)] = blocks[z + ((63 - y) * 64) + (x * 64 * 64)]
            }
        }
    }

    level.blocks = Some(blocks1);
    //println!("{}",level.blocks.unwrap().len()/64/64);

    //Setting world size
    level.width = Some(64);
    level.depth = Some(64);
    level.height = Some(64);

    return mc_classic::level_to_classic_13(level, "level.dat".to_string());
}


pub fn write_4k (input: &str, mode: u8) {

    let level: mc_classic::Level = mc_classic::read_level(input.to_string())
        .expect("You idiot, you gave me the wrong file");

    //Converting block ids
    let mut bytes: Vec<u8> = Vec::new();

    for block in level.blocks.unwrap() {

        if mode == 0 {
            if block == 0 {bytes.push(0)} else {bytes.push(1)}
        } else {
            match block {
                0 => bytes.push(0), //Air
                1 => bytes.push(4), //Stone                
                2 => bytes.push(1), //Grass
                3 => {bytes.push(2) /*Randomize dirt, 2 3 6 9-15*/}, //Dirt
                9 => if mode != 2 {bytes.push(2) /*Dirt*/} else {bytes.push(9 /*Water*/)}, 
                17 => bytes.push(7), //Logs
                18 => bytes.push(8), //Leaves
                45 => bytes.push(5), //Bricks
                _ => bytes.push(4)
            }
        }
    }

    let mut h: usize = if level.height.is_some() {level.height.unwrap() as usize} else {64};
    let mut d: usize = if level.depth.is_some() {level.depth.unwrap() as usize} else {64};
    let mut w: usize = if level.width.is_some() {level.width.unwrap() as usize} else {64};

    //Pruning world
    let mut bytes1: Vec<u8> = Vec::new();

    for y in 0..d {
        for z in 0..h {
            for x in 0..w {
                if y >= 64 || z >= 64 || x >= 64 {continue}
                bytes1.push(bytes[x + (z * w) + (y * w * h)])
            }
        }
    }

    if h > 64 {h = 64}
    if d > 64 {d = 64}
    if w > 64 {w = 64}

    //Fixing positioning of the world
    let mut bytes2: Vec<u8> = vec![0; w*d*h];

    for x in 0..w{
        for y in 0..d{
            for z in 0..h{
                bytes2[z + ((d - 1 - y) * h) + (x * d * h)] = bytes1[x + (z * w) + (y * w * h)]
            }
        }
    }

    //Converting byte array to int array
    let mut bytes3: Vec<u8> = Vec::new();

    for byte in bytes2 {
        if mode != 2 {
            for _ in 0..3 {bytes3.push(0)}
        }
        bytes3.push(byte);
    }

    //Writing world
    let mut output= OpenOptions::new()
        .write(true)
        .create(true)
        .open("level.4k")
        .unwrap();

    if mode != 2 {
        let mut encoder = GzEncoder::new(output, Compression::default());
        encoder.write_all(&bytes3).unwrap();
    } else {
        output.write(&bytes3);
    }

}

