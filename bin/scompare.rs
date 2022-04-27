use clap::{Parser, ArgEnum};
use rendering::colourmap::Colourmap;

use rendering::Float;
use rendering::image::ImageBuffer;


#[derive(ArgEnum, Clone)]
enum ArgColourMap {
    Radiance,
    Inferno,
    Magma,
    Plasma,
    Viridis
}



#[derive(Parser)]
struct Inputs {
    #[clap(short='a')]
    /// One of the images to be compared
    pub input1: String,

    #[clap(short='b')]
    /// The other image to compare
    pub input2: String,

    #[clap(short, long)]
    /// The output file (it is an HDRE if no colourmap (i.e., -m <Map>); 
    /// otherwise it is falsecolored and stored in JPEG format
    pub output: String,

    /// The maximum value in the scale
    #[clap(short='s', long)]
    pub max: Option<Float>,

    /// The minimum value in the scale
    #[clap(long)]
    pub min: Option<Float>,

    /// Use a log10 scale
    #[clap(short, long)]    
    pub log: bool,    

    /// The colour scale to use
    #[clap(arg_enum, short, long)]    
    map: Option<ArgColourMap>,
}


fn main(){
    let inputs = Inputs::parse();

    

    let a = match ImageBuffer::from_file(std::path::Path::new(&inputs.input1)){
        Ok(v)=>v,
        Err(e)=>{
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let b = match ImageBuffer::from_file(std::path::Path::new(&inputs.input2)){
        Ok(v)=>v,
        Err(e)=>{
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    
    let diff = match b.diff(&a){
        Ok(v)=>v,
        Err(e)=>{
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let out = std::path::Path::new(&inputs.output);

    match inputs.map {
        Some(s)=>{
            let scale = match s {
                ArgColourMap::Radiance => Colourmap::Radiance,
                ArgColourMap::Inferno => Colourmap::Inferno,
                ArgColourMap::Magma => Colourmap::Magma,
                ArgColourMap::Plasma => Colourmap::Plasma,
                ArgColourMap::Viridis => Colourmap::Viridis,
            };
            if inputs.log {
                diff.save_log_falsecolour(inputs.min, inputs.max, scale, out)
            }else{
                diff.save_falsecolour(inputs.min, inputs.max, scale, out)
            }
        },
        None => {
            diff.save_hdre(out);
        }
    }

    

    
    
}