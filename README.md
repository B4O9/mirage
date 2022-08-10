
# Mirage

You can use this program to edit your image in various ways.




## Usage/Examples
Run the following command to help you get started and see the features:
```cli
cargo run -- -help
```
For example, to blur an image you can use the following:
```
cargo run -- -i duck.png -o duck_blurred.png blur 2.5
```

Apart from taking input and output file from the user, some of the features need more input:
```
blur(infile, outfile, blur_amount(float)),
brighten(infile, outfile, bright_amount),
crop(infile, outfile, x, y, width, height),
rotate(infile, outfile, rotation),
generate(outfile, width, height),
fractal(outfile, width, height)
```
## Features

- Blur
- Brighten
- Crop
- Invert
- Rotate (90, 180, 270)
- Generate
- Fractal
- Grayscale

Also, if you do not want to write "cargo run --" everytime, you can change your workspace to target/debug and run:
```
mirage -i duck.png -o duck_cropped.png crop 6 9 4 20 
```

## Requirements
Rust, Cargo.
## Roadmap

- Will add stackable commands. For example:
```
cargo run -- -i duck.png -o duck1.png blur 2.5 brighten 200 rotate 270
```

