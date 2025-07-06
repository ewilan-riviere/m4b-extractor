use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input audiobook file (positional argument)
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Output directory (optional, default is input_name + "_chapters")
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Keep m4b format
    #[arg(short = 'k', long = "keep", default_value_t = false)]
    pub no_convert: bool,

    /// Conversion quality (1=best, 9=worst)
    #[arg(short = 'q', long, default_value_t = 2)]
    pub quality: u8,

    /// Sanitize filenames (default: false)
    /// This option replaces invalid characters with underscores
    #[arg(short = 's', long = "sanitize", default_value_t = false)]
    pub sanitize: bool,
}
