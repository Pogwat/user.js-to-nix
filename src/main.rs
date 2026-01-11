use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    //first arg is pname, rest are file args, they must be space spereated
    // ff-format input_file output_file
    let args: Vec<String> = env::args().collect();
    let (input_file_name, output_file_name) = if args.len() == 3 {
        (&args[1], &args[2])
    }
    else { //if incorrect arg number exit and print usage
        eprintln!{
"ff-format formats firefox user.js userprefs to nix
Will Overide output_file !!
Must use absolue or relative paths no ~ or env varibles
Usage: 
ff-format [input_file] [output_file]
Example Usage: 
ff-format user.js prefs.nix
ff-format /home/pog/ff-format/src/user.js /home/pog/nix_userjs.nix"
        };
        return Ok(()); //exit
    };

    //Input file read buffer
    let input_file = File::open(&input_file_name)?; 
    let reader = BufReader::new(input_file); 
    
    //Output file write buffer
    let output_file = File::create(&output_file_name)?;
    let mut output_buffer = BufWriter::new(output_file);
    
    // { To output Buffer
    writeln!(output_buffer,"{{")?;

    //line by line parsing of input file
    // if userpref, name=value; 
    for line in reader.lines() {
        let line = line?;
        //Check if line starts with userpref( if so strip it and return &str
        if let Some(start) = &line.strip_prefix("user_pref(") { //9 long
            //Find first occurence of ); to get length of stuff inside userpref   
            if let Some(end) = start.find(");"){   //byte's start at 0
                //string slice of only stuff betwen userpref( and );
                let output = &start[..end]; 
                //Split the stuff between userpref( and ); that is quote seperated
                //assign the stuff before quote name and stuff after value
                if let Some((name,value)) = output.split_once(",") {
                    //name=value;
                    writeln!(output_buffer, "{} ={};", &name, &value)?;
                }
            }
        }
        else {
            //append # to everything besides userpref
            writeln!(output_buffer, "#{}", &line)?; 
        }
    
   }
   // } To output Buffer
   // { ....userprefs }, for nix formatting as attribute set
   writeln!(output_buffer,"}}")?;
   //Write the buffer to the output file
   output_buffer.flush()?;  
   Ok(())
   
}