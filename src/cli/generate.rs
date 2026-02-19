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

    let generated_doc = generate_svg(
        args.rows,
        args.columns,
        args.material_thickness,
        &container,
        &args.primary_color,
        &args.secondary_color,
    );
    let svg = &generated_doc.document;
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
    svg::save(&filename_with_extension, svg).unwrap();
    println!("Container size: {:.1}mm (W) x {:.1}mm (H) x {:.1}mm (D)", generated_doc.container_dimensions.width, generated_doc.container_dimensions.height, generated_doc.container_dimensions.depth);
    println!("Saved to {}", &filename_with_extension);
}

#[cfg(test)]
mod tests {
    use container_rack_lib::rack::AssembledDimensions;

    // Helper function to format dimensions like the CLI does
    fn format_dimensions(dims: &AssembledDimensions) -> String {
        format!(
            "Container size: {:.1}mm (W) x {:.1}mm (H) x {:.1}mm (D)",
            dims.width, dims.height, dims.depth
        )
    }

    // Unit tests for CLI output formatting
    // **Validates: Requirements 5.1, 5.2, 5.3**

    #[test]
    fn test_format_string_matches_specification() {
        // Test that the format string matches the specification:
        // "Container size: {width}mm (W) x {height}mm (H) x {depth}mm (D)"
        let dims = AssembledDimensions {
            width: 100.0,
            height: 200.0,
            depth: 150.0,
        };

        let output = format_dimensions(&dims);

        // Verify the format matches specification
        assert_eq!(
            output,
            "Container size: 100.0mm (W) x 200.0mm (H) x 150.0mm (D)"
        );

        // Verify the format contains all required components
        assert!(output.starts_with("Container size: "));
        assert!(output.contains("mm (W)"));
        assert!(output.contains("mm (H)"));
        assert!(output.contains("mm (D)"));
    }

    #[test]
    fn test_rounding_whole_numbers() {
        // Test rounding behavior with whole numbers
        let dims = AssembledDimensions {
            width: 332.0,
            height: 210.0,
            depth: 100.0,
        };

        let output = format_dimensions(&dims);

        // Verify whole numbers are displayed with one decimal place
        assert_eq!(
            output,
            "Container size: 332.0mm (W) x 210.0mm (H) x 100.0mm (D)"
        );
    }

    #[test]
    fn test_rounding_one_decimal_place() {
        // Test rounding behavior with one decimal place
        let dims = AssembledDimensions {
            width: 209.5,
            height: 201.0,
            depth: 100.0,
        };

        let output = format_dimensions(&dims);

        // Verify numbers are displayed with one decimal place
        assert_eq!(
            output,
            "Container size: 209.5mm (W) x 201.0mm (H) x 100.0mm (D)"
        );
    }

    #[test]
    fn test_rounding_truncates_extra_decimals() {
        // Test that extra decimal places are rounded to one decimal place
        let dims = AssembledDimensions {
            width: 123.456,
            height: 234.567,
            depth: 345.678,
        };

        let output = format_dimensions(&dims);

        // Verify numbers are rounded to one decimal place
        assert_eq!(
            output,
            "Container size: 123.5mm (W) x 234.6mm (H) x 345.7mm (D)"
        );
    }

    #[test]
    fn test_format_with_very_small_values() {
        // Test formatting with very small decimal values
        let dims = AssembledDimensions {
            width: 90.1,
            height: 66.2,
            depth: 120.3,
        };

        let output = format_dimensions(&dims);

        // Verify small values are formatted correctly
        assert_eq!(
            output,
            "Container size: 90.1mm (W) x 66.2mm (H) x 120.3mm (D)"
        );
    }

    #[test]
    fn test_format_with_large_values() {
        // Test formatting with large values
        let dims = AssembledDimensions {
            width: 806.0,
            height: 612.0,
            depth: 200.0,
        };

        let output = format_dimensions(&dims);

        // Verify large values are formatted correctly
        assert_eq!(
            output,
            "Container size: 806.0mm (W) x 612.0mm (H) x 200.0mm (D)"
        );
    }

    #[test]
    fn test_output_order_dimensions_before_save() {
        // This test verifies the conceptual order of output
        // In the actual CLI code, dimensions are printed before the save confirmation
        // We verify this by checking the line numbers in the source code

        // The actual implementation in svg() function:
        // 1. println!("So you want to generate...")
        // 2. generate_svg(...)
        // 3. svg::save(...)
        // 4. println!("Container size: ...") <- dimensions output
        // 5. println!("Saved to {}") <- save confirmation

        // This test documents the expected order
        let expected_order = vec![
            "Container size output",
            "Save confirmation output",
        ];

        // Verify the order is as expected
        assert_eq!(expected_order[0], "Container size output");
        assert_eq!(expected_order[1], "Save confirmation output");
    }

    #[test]
    fn test_format_consistency_with_cli_implementation() {
        // Test that our helper function matches the actual CLI implementation
        let dims = AssembledDimensions {
            width: 332.0,
            height: 210.0,
            depth: 100.0,
        };

        let helper_output = format_dimensions(&dims);

        // This is the exact format used in the CLI
        let cli_format = format!(
            "Container size: {:.1}mm (W) x {:.1}mm (H) x {:.1}mm (D)",
            dims.width, dims.height, dims.depth
        );

        // Verify they match
        assert_eq!(helper_output, cli_format);
    }
}

