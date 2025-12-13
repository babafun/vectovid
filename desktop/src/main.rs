use vvf::version;

fn main() {
    println!("vectovid desktop - VVF player and packer");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Using vvf core library version: {}", version());
    
    println!("\nUsage:");
    println!("  vectovid [COMMAND]");
    println!("\nCommands:");
    println!("  pack    - Pack SVG frames into a .vvf file");
    println!("  play    - Play a .vvf video file");
    println!("  help    - Show this help message");
}
