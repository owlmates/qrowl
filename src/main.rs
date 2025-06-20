use clap::Parser;
use image::Luma;
use qrcode::QrCode;
use std::process;

#[derive(Parser)]
#[command(name = "qrowl")]
#[command(about = "A simple QR code generator")]
#[command(version)]
struct Args {
    text: String,
    #[arg(short, long, default_value = "qrcode.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let code = match QrCode::new(&args.text) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: Failed to generate QR code: {}", e);
            process::exit(1);
        }
    };

    let image = code.render::<Luma<u8>>().build();
    if let Err(e) = image.save(&args.output) {
        eprintln!("Error: Failed to save image to '{}': {}", args.output, e);
        process::exit(1);
    }

    println!("QR code successfully generated and saved to '{}'", args.output);
}
