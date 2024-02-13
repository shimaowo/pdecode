use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        return Ok(());
    }

    // Path is really inefficient for basically anything, but I don't want to deal with edge cases around . etc
    let input_path = Path::new(&args[1]);
    let mut output_path = PathBuf::from(input_path).with_file_name(format!(
        "{}_new",
        input_path.file_stem().unwrap().to_string_lossy()
    ));
    if let Some(ext) = input_path.extension() {
        output_path.set_extension(ext);
    }

    let f = File::open(input_path)?;
    let mut outfile = File::create(output_path)?;

    let reader = BufReader::new(f);
    for (idx, v) in reader
        .split(b'%')
        .skip(1)
        .map(|res| res.unwrap())
        .enumerate()
    {
        let len = v.len();
        if len == 2 {
            let s = std::str::from_utf8(&v).expect("failed utf parse");
            let num = match u8::from_str_radix(s, 16) {
                Ok(val) => val,
                Err(e) => panic!("failed hex parse {}", e),
            };
            outfile.write_all(&num.to_le_bytes())?;
        } else {
            eprint!("Skipping parse of invalid chunk at offset {}: %", idx);
            for c in &v {
                eprint!("{}", *c as char);
            }
            eprintln!();

            // just write out any invalid data verbatim
            let x: [u8; 1] = [b'%'];
            outfile.write_all(&x)?;
            outfile.write_all(&v)?;
        }
    }

    Ok(())
}
