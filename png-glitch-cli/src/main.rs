use clap::Parser;
use png_glitch::PngGlitch;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, default_value = "glitched.png")]
    pub output_file: String,
    pub input_file: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(&cli) {
        println!("{:?}", e);
    }
}

fn start(cli: &Cli) -> anyhow::Result<()> {
    let mut glitch = PngGlitch::open(&cli.input_file)?;
    run(&mut glitch);
    glitch.save(&cli.output_file)?;
    Ok(())
}

fn run(glitch: &mut PngGlitch) {
    glitch.foreach_scanline(|scanline| {
        scanline.update(3, 0);
    });
}
