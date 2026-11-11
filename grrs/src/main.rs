use clap::Parser;  
use anyhow::{Context, Result};


#[derive(Parser)]
struct Cli {
    // The patten to find for
    pattern: String, 
    // The path to the file to read
    path: std::path::PathBuf, 
}


fn main() -> Result<()>{
    // if i don't you struct
    // let pattern = args().nth(1).expect("no pattern given"); 
    // let path = args().nth(2).expect("no path given"); 

   
    // let args = Cli::parse();
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path); 

    let args = Cli::parse(); 

    // read_to_string ->  reads its entire contents into a standard String - low performance.
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    

        for line in content.lines() { 
            if line.contains(&args.pattern){
                println!("{}", line);
            }
        }

        Ok(())
}
