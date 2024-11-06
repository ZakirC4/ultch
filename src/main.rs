use std::fs::OpenOptions;
use std::io::{self, BufReader, BufRead, Read, Seek, SeekFrom, Write};
use std::{path::Path, env};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]).join("=").join("cmd.cmd");
    let target_line = "command = ~D, DF, F, D, DF, F, a+b";
    let replacement_line = "command = a+x";
    let num_lines = 100;

    // Open the file with read and write access
    let mut file = OpenOptions::new().read(true).write(true).open(&file_path)?;
    let mut reader = BufReader::new(&file);

    let mut buffer = Vec::new();
    let mut line_found = false;
    let mut line_position = 0;

    // Read up to `num_lines` lines, looking for the target line
    for (i, line) in reader.by_ref().lines().enumerate() {
        let line = line?;
        if line == target_line && i < num_lines {
            // Replace the target line with the new content
            buffer.push(replacement_line.to_string());
            line_position = i;
            line_found = true;
        } else {
            buffer.push(line);
        }
        if i >= num_lines - 1 {
            break;
        }
    }

    if line_found {
        // Seek to the beginning of the file to start rewriting
        file.seek(SeekFrom::Start(0))?;

        // Rewrite the buffered lines
        for line in &buffer {
            writeln!(file, "{}", line)?;
        }

        // Get the current position to set the file length
        let current_pos = file.seek(SeekFrom::Current(0))?;

        // Truncate the file to avoid leftover content
        file.set_len(current_pos)?;

        println!("Successfully replaced line {}", line_position + 1);
    } else {
        println!("Target line not found in the first {} lines", num_lines);
    }

    Ok(())
}
