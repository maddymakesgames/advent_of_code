use std::{fs::OpenOptions, io::Read};

fn main() -> anyhow::Result<()> {
    // Read the input file into a string
    let mut input_file = OpenOptions::new().read(true).open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut elves = Vec::new();
    let mut curr_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            // If the line is empty we have reached the end of an elf
            elves.push(curr_elf);
            curr_elf = 0;
        } else {
            // Parse the string into an int and add it to our current total
            curr_elf += line.parse::<u32>()?;
        }
    }

    // Sort the entries
    elves.sort_by(|a, b| b.cmp(a));

    // Print out the largest
    println!("{:?}", elves.first());

    Ok(())
}
