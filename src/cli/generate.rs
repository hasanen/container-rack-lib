use clap::Parser;
use container_rack_lib::{generate_svg, supported_containers};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RackGenerationArgs {
    /// Number of rows of boxes
    #[arg(short, long)]
    rows: usize,

    /// Number columns of boxes
    #[arg(short, long)]
    columns: usize,

    /// Thickness of the plywood or other material
    #[arg(short, long)]
    material_thickness: f32,

    /// Key of container
    #[arg(long)]
    container: String,

    /// Name of the file to save the SVG to
    #[arg(short, long)]
    output_filename: Option<String>,

    /// Primary color of the line that will be cut first
    #[clap(short, long, default_value = "black")]
    primary_color: String,

    /// Primary color of the line that will be cut first
    #[clap(short, long, default_value = "blue")]
    secondary_color: String,
}

/// Generate SVG for the container rack
pub fn svg(args: &RackGenerationArgs) {
    println!(
        "So you want to generate organizer with {} rows and {} columns, using {}mm thick material.",
        args.rows, args.columns, args.material_thickness
    );
    let supported_containers = supported_containers();

    let container = match supported_containers
        .iter()
        .find(|c| c.key() == args.container)
    {
        Some(container) => container,
        None => {
            println!("No supported containers found.");
            //exit from process
            std::process::exit(1);
        }
    };

    let svg = generate_svg(
        args.rows,
        args.columns,
        args.material_thickness,
        &container,
        &args.primary_color,
        &args.secondary_color,
    );
    let filename = match args.output_filename.clone() {
        Some(name) => name,
        None => format!(
            "organizer_{}_rows_{}_columns_{}mm_thick_{}",
            args.rows,
            args.columns,
            args.material_thickness,
            container.key()
        ),
    };
    let filename_with_extension = format!("{}.svg", filename);
    svg::save(&filename_with_extension, &svg).unwrap();
    println!("Saved to {}", &filename_with_extension);
}
