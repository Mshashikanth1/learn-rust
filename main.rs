use clap::Parser;
use anyhow::{Context, Result};
#[derive(Parser)]
struct Cli{
    pattern:String,
    path:std::path::PathBuf,
}

fn main() -> Result<()> {
/*cmdlie argument let arg1=std::env::args().nth(1).expect("not pattern given")*/ 
    let _args=Cli::parse();
   /* print!("{:#?} , {:#?}",_args.pattern,_args.path);*/
    let _content=std::fs::read_to_string(&_args.path)
        .with_context(|| format!("could not read file"))?;
/*
    for line in _content.lines(){
        if line.contains(&_args.pattern){
            println!("{}",line);
        }
    }
  */

    println!("file content:{}",_content);
    Ok(())
}
