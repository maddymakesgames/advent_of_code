use std::{fs::OpenOptions, io::Read};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Item(char);
impl Item {
    fn priority(&self) -> u32 {
        if ('a' ..= 'z').contains(&self.0) {
            self.0 as u8 as u32 - 96
        } else {
            self.0 as u8 as u32 - 38
        }
    }
}


fn main() -> Result<()> {
    let mut input_file = OpenOptions::new().read(true).open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut total_priority = 0;
    let mut lines = input.lines();

    while let Some(a) = lines.next() {
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        let compartment_a = a.chars().map(Item).collect::<Vec<_>>();
        let compartment_b = b.chars().map(Item).collect::<Vec<_>>();
        let compartment_c = c.chars().map(Item).collect::<Vec<_>>();

        'a_loop: for a in &compartment_a {
            for b in &compartment_b {
                for c in &compartment_c {
                    if a == b && a == c {
                        total_priority += a.priority();
                        break 'a_loop;
                    }
                }
            }
        }
    }

    println!("{total_priority}");

    Ok(())
}
