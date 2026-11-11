use clap::Parser; 

#[derive(Parser)]
struct Cli {
    // The patten to find for
    pattern: String, 
    // The path to the file to read
    path: std::path::PathBuf, 
}



fn main()-> Return<(), Box<dyn std::error::Error >> {
    // if i don't you struct
    // let pattern = args().nth(1).expect("no pattern given"); 
    // let path = args().nth(2).expect("no path given"); 

   
    // let args = Cli::parse();
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path); 

    let args = Cli::parse(); 

    // read_to_string ->  reads its entire contents into a standard String - low performance.
    let content = std::fs::read_to_string(&args.path).expect("couldn't real file");
    let content = match result {
        Ok(content) => { content }, 
        // Err(error) => { panic! ("Can't deal with {}, just exit here", error); }
        Err(error) => {return Err (error.into()); }
    }
        println!("file content: {}", content);

        for line in content.lines() { 
            if line.contains(&args.pattern){
                println!("{}", line);
            }
        }
}
