use mirage::{blur, brighten, crop, rotate, invert, grayscale, generate, fractal};
mod args;
use args::CommandType;
use args::MirageArgs;
use clap::Parser;

fn main() {
    
    let args: MirageArgs = MirageArgs::parse();
    let infile = args.input_file;
    let outfile = args.output_file;

    match args.command {

        CommandType::Blur(cmd) => blur(infile, outfile, cmd.blur_amount),
        CommandType::Bright(cmd) => brighten(infile, outfile, cmd.bright_amount),
        CommandType::Crop(cmd) => crop(infile, outfile, cmd.x, cmd.y, cmd.width, cmd.height),
        CommandType::Rotate(cmd) => rotate(infile, outfile, cmd.rotation),
        CommandType::Grayscale => grayscale(infile, outfile),
        CommandType::Invert => invert(infile, outfile),
        CommandType::Generate(cmd) => generate(outfile, cmd.width, cmd.height),
        CommandType::Fractal(cmd) => fractal(outfile, cmd.width, cmd.height),

    }
}