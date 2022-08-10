use clap::{Args, Parser, Subcommand};
     
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MirageArgs {
    /// Input file to be edited
    #[clap(short, long, value_parser, default_value = "")]
    pub input_file: String,
    /// Output file to save value
    #[clap(short, long, value_parser, default_value = "mod.png")]
    pub output_file: String,
    #[clap(subcommand)]
    pub command: CommandType,
}
 
#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Blur image
    Blur(BlurCommand),
    /// Brightens image
    Bright(BrightCommand),
    /// Crop image
    Crop(CropCommand),
    /// Rotate image
    Rotate(RotateCommand),
    /// Invert image
    Invert,
    /// Transform image to grayscale
    Grayscale,
    /// Perform fractal on image
    Fractal(FractalCommand),
    /// Generate image
    Generate(GenerateCommand),
}
 
#[derive(Debug, Args)]
pub struct BlurCommand {
    pub blur_amount: f32,
}
 
#[derive(Debug, Args)]
pub struct BrightCommand {
    pub bright_amount: i32,
}
 
#[derive(Debug, Args)]
pub struct CropCommand {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
 
#[derive(Debug, Args)]
pub struct RotateCommand {
    pub rotation: u32,
}

#[derive(Debug, Args)]
pub struct FractalCommand {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Args)]
pub struct GenerateCommand {
    pub width: u32,
    pub height: u32,
}