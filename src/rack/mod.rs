use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::{Document, Node};
use url::Url;

// All measurements are in mm
const SIDE_WING_SLOT_FROM_FRONT: usize = 20;
const SIDE_WING_SLOT_WIDTH: usize = 20;
const SIDE_WING_SLOT_SPACING: usize = 15;
const CLEARANCE_BETWEEN_PATHS: usize = 3;
const SIDE_TAP_FROM_FRONT: usize = 30;
const SIDE_TAP_WIDTH: usize = 30;
const CLEARANCE_FOR_CONTAINER_WIDTH: usize = 4;

#[derive(Debug, Clone)]
pub struct Container {
    pub vendor: String,
    pub model: String,
    pub description: String,
    pub links: Vec<ContainerLink>,
    pub dimensions: ContainerDimensions,
}

impl Container {
    pub fn key(&self) -> String {
        format!("{}-{}", self.vendor, self.model)
            .to_lowercase()
            .replace(" ", "_")
    }
}

#[derive(Debug, Clone)]
pub struct ContainerDimensions {
    pub width: usize,
    pub depth: usize,
    pub height: usize,
    pub side_wing_from_box_top: usize,
    pub side_wing_width: usize,
}

#[derive(Debug, Clone)]
pub struct ContainerLink {
    pub url: Url,
    pub title: String,
}
#[derive(Debug, Clone)]
pub struct AssembledDimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32
}
#[derive(Debug, Clone)]
pub struct GeneratedSvg {
    pub document: Document,
    pub assembled_dimensions: AssembledDimensions,
}
pub fn generate_svg(
    rows: usize,
    columns: usize,
    material_thickness: f32,
    container: &Container,
    primary_color: &str,
    secondary_color: &str,
) -> GeneratedSvg {
    let starting_point_x = 0.0;
    let starting_point_y = 0.0;
    let column_width = container.dimensions.width + CLEARANCE_FOR_CONTAINER_WIDTH;
    let amount_of_boxes = (rows * columns) as usize;
    let height_of_two_side_wings =
        height_of_two_side_wings(container.dimensions.side_wing_width, material_thickness);
    let height_of_two_side_wings_with_clearance =
        height_of_two_side_wings + CLEARANCE_BETWEEN_PATHS as f32;

    let total_width = (container.dimensions.depth + (CLEARANCE_BETWEEN_PATHS * 3)) as f32
        + top_width(column_width as f32, columns, material_thickness)
        + (container.dimensions.height * rows) as f32
        + (2.0 * material_thickness);
    let total_height = vec![
        amount_of_boxes as f32 * height_of_two_side_wings_with_clearance,
        (2 * container.dimensions.depth + CLEARANCE_BETWEEN_PATHS) as f32,
        ((columns + 1) * (container.dimensions.depth + CLEARANCE_BETWEEN_PATHS)) as f32,
    ]
    .iter()
    .cloned()
    .fold(f32::NEG_INFINITY, f32::max);

    let mut document = Document::new()
        .set("viewBox", (0, 0, total_width, total_height))
        .set("width", format!("{}mm", total_width))
        .set("height", format!("{}mm", total_height));

    // Generate side wings
    for i in 0..amount_of_boxes {
        generate_side_wing_pair(
            &mut document,
            &container.dimensions,
            starting_point_x,
            starting_point_y + height_of_two_side_wings_with_clearance * i as f32,
            material_thickness,
            secondary_color,
        );
    }

    // Generate top and bottom pieces
    generate_top_and_bottom_pieces(
        &mut document,
        &container.dimensions,
        (container.dimensions.depth + CLEARANCE_BETWEEN_PATHS) as f32,
        columns,
        column_width as f32 + material_thickness,
        material_thickness,
        primary_color,
        secondary_color,
    );

    // generate side panels
    generate_side_panels(
        &mut document,
        (container.dimensions.depth + CLEARANCE_BETWEEN_PATHS) as f32 //side wings
            + top_width(column_width as f32, columns, material_thickness) + CLEARANCE_BETWEEN_PATHS as f32,
        &container.dimensions, // top and bottom plates
        rows,
        columns,
        material_thickness,
        primary_color,
        secondary_color,
    );

    // Calculate assembled dimensions
    let assembled_width = (column_width * columns) as f32 
        + (columns + 1) as f32 * material_thickness;
    
    let assembled_height = (container.dimensions.height * rows) as f32 
        + material_thickness * 2.0;
    
    let assembled_depth = container.dimensions.depth as f32;
    
    GeneratedSvg {
        document,
        assembled_dimensions: AssembledDimensions {
            width: assembled_width,
            height: assembled_height,
            depth: assembled_depth
        }
    }
}

fn generate_side_panels(
    document: &mut Document,
    starting_point_x: f32,
    dimensions: &ContainerDimensions,
    rows: usize,
    columns: usize,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    for i in 0..columns + 1 {
        let y = (i * (dimensions.depth + CLEARANCE_BETWEEN_PATHS)) as f32;

        document.append(generate_side_panel_outline_path(
            starting_point_x,
            y,
            dimensions,
            rows,
            material_thickness,
            secondary_color,
        ));

        for r in 0..rows {
            let row_x = material_thickness
                + (dimensions.side_wing_from_box_top + r * dimensions.height) as f32;

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + SIDE_WING_SLOT_FROM_FRONT as f32,
                material_thickness,
                primary_color,
            ));

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH + SIDE_WING_SLOT_SPACING)
                    as f32,
                material_thickness,
                primary_color,
            ));

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (dimensions.depth
                    - SIDE_WING_SLOT_FROM_FRONT
                    - (2 * SIDE_WING_SLOT_WIDTH)
                    - SIDE_WING_SLOT_SPACING) as f32,
                material_thickness,
                primary_color,
            ));
            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (dimensions.depth - SIDE_WING_SLOT_FROM_FRONT - SIDE_WING_SLOT_WIDTH) as f32,
                material_thickness,
                primary_color,
            ));
        }
    }
}

fn generate_side_panel_wing_holes(x: f32, y: f32, material_thickness: f32, color: &str) -> Path {
    let path_data = Data::new()
        .move_to((x, y))
        .vertical_line_to(y + SIDE_WING_SLOT_WIDTH as f32)
        .horizontal_line_to(x + material_thickness)
        .vertical_line_to(y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", path_data)
}

fn generate_side_panel_outline_path(
    starting_point_x: f32,
    starting_point_y: f32,
    dimensions: &ContainerDimensions,
    rows: usize,
    material_thickness: f32,
    color: &str,
) -> Path {
    let panel_inner_height = (dimensions.height * rows) as f32;
    let side_panel_path_data = Data::new()
        .move_to((starting_point_x + material_thickness, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(
            starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT - SIDE_TAP_WIDTH) as f32,
        )
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + dimensions.depth as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (2.0 * material_thickness))
        .vertical_line_to(
            starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT - SIDE_TAP_WIDTH) as f32,
        )
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (2.0 * material_thickness))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", side_panel_path_data)
}

fn generate_top_and_bottom_pieces(
    document: &mut Document,
    dimensions: &ContainerDimensions,
    starting_point_x: f32,
    columns: usize,
    column_width: f32,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    generate_cover_path(
        document,
        dimensions,
        starting_point_x,
        0.0,
        columns,
        column_width,
        material_thickness,
        primary_color,
        secondary_color,
    );

    generate_cover_path(
        document,
        dimensions,
        starting_point_x,
        (dimensions.depth + CLEARANCE_BETWEEN_PATHS) as f32,
        columns,
        column_width,
        material_thickness,
        primary_color,
        secondary_color,
    );
}

fn generate_cover_path(
    document: &mut Document,
    dimensions: &ContainerDimensions,
    starting_point_x: f32,
    starting_point_y: f32,
    columns: usize,
    column_width: f32,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    // Generate cover
    let top_path_data = generate_top_path(
        dimensions,
        starting_point_x,
        starting_point_y,
        columns,
        column_width,
        material_thickness,
    );
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", secondary_color)
        .set("d", top_path_data);
    document.append(path);

    for i in 0..columns - 1 {
        let x = starting_point_x + column_width + (i as f32 * column_width);
        let y = starting_point_y + SIDE_TAP_FROM_FRONT as f32;
        let side_tap_hole_path = generate_side_tap_path(x, y, material_thickness, primary_color);
        document.append(side_tap_hole_path);

        let side_tap_hole_path = generate_side_tap_path(
            x,
            y + (dimensions.depth - SIDE_TAP_FROM_FRONT - (SIDE_TAP_WIDTH * 2)) as f32,
            material_thickness,
            primary_color,
        );
        document.append(side_tap_hole_path);
    }

    //Generate side panel taps to middle of cover
}

fn generate_side_tap_path(x: f32, y: f32, material_thickness: f32, color: &str) -> Path {
    let data = Data::new()
        .move_to((x, y))
        .vertical_line_to(y + SIDE_TAP_WIDTH as f32)
        .horizontal_line_to(x + material_thickness as f32)
        .vertical_line_to(y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", data)
}

fn generate_top_path(
    dimensions: &ContainerDimensions,
    starting_point_x: f32,
    starting_point_y: f32,
    columns: usize,
    column_width: f32,
    material_thickness: f32,
) -> Data {
    let top_width = top_width(column_width, columns, material_thickness);

    Data::new()
        .move_to((starting_point_x, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(
            starting_point_y + (dimensions.depth - (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH)) as f32,
        )
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + dimensions.depth as f32)
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y + (dimensions.depth - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x - material_thickness + top_width)
        .vertical_line_to(
            starting_point_y + (dimensions.depth - (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH)) as f32,
        )
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x - material_thickness + top_width)
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y)
        .close()
}

fn top_width(column_width: f32, columns: usize, material_thickness: f32) -> f32 {
    (material_thickness + column_width * columns as f32) + material_thickness
}
fn generate_side_wing_pair(
    document: &mut Document,
    dimensions: &ContainerDimensions,
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    color: &str,
) {
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y,
        material_thickness,
        dimensions.depth,
        dimensions.side_wing_width,
        false,
        &color,
    );
    document.append(path);
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y + (dimensions.side_wing_width + CLEARANCE_BETWEEN_PATHS) as f32,
        material_thickness,
        dimensions.depth,
        dimensions.side_wing_width,
        true,
        &color,
    );
    document.append(path);
}

fn height_of_two_side_wings(side_wing_width: usize, material_thickness: f32) -> f32 {
    (side_wing_width * 2 + CLEARANCE_BETWEEN_PATHS) as f32 + material_thickness
}

fn generate_side_wing(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    box_depth: usize,
    box_side_wing_width: usize,
    inverted: bool,
    color: &str,
) -> Path {
    let wing_data = if inverted {
        generate_side_wing_inverted_path(
            starting_point_x,
            starting_point_y,
            material_thickness,
            box_depth,
            box_side_wing_width,
        )
    } else {
        generate_side_wing_path(
            starting_point_x,
            starting_point_y,
            material_thickness,
            box_depth,
            box_side_wing_width,
        )
    };

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", wing_data)
}

fn generate_side_wing_path(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    box_depth: usize,
    box_side_wing_width: usize,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y))
        .vertical_line_to(starting_point_y + box_side_wing_width as f32)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + material_thickness + box_side_wing_width as f32)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + box_side_wing_width as f32)
        .horizontal_line_to(third_side_wing_tap_position_from_front(box_depth))
        .vertical_line_to(starting_point_y + box_side_wing_width as f32 + material_thickness)
        .horizontal_line_to(
            third_side_wing_tap_position_from_front(box_depth) + SIDE_WING_SLOT_WIDTH,
        )
        .vertical_line_to(starting_point_y + box_side_wing_width as f32)
        .horizontal_line_to(box_depth)
        .vertical_line_to(starting_point_y)
        .close()
}

fn generate_side_wing_inverted_path(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    box_depth: usize,
    box_side_wing_width: usize,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y + material_thickness))
        .horizontal_line_to(second_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(second_side_wing_tap_position_from_front() + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(fourth_side_wing_tap_position_from_front(box_depth))
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(box_depth - SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(box_depth)
        .vertical_line_to(starting_point_y + material_thickness + box_side_wing_width as f32)
        .horizontal_line_to(starting_point_x)
        .close()
}

fn third_side_wing_tap_position_from_front(box_depth: usize) -> usize {
    box_depth
        - (SIDE_WING_SLOT_FROM_FRONT
            + SIDE_WING_SLOT_WIDTH
            + SIDE_WING_SLOT_SPACING
            + SIDE_WING_SLOT_WIDTH)
}
fn second_side_wing_tap_position_from_front() -> usize {
    SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH + SIDE_WING_SLOT_SPACING
}

fn fourth_side_wing_tap_position_from_front(box_depth: usize) -> usize {
    box_depth - (SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: calculate-assembled-dimensions, Property 1: Assembled Width Formula
    // **Validates: Requirements 1.1, 1.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_assembled_width_formula(
            columns in 1usize..=10,
            container_width in 50usize..=500,
            material_thickness in 1.0f32..=20.0,
        ) {
            // Create a minimal container with the generated dimensions
            let container = Container {
                vendor: "Test".to_string(),
                model: "Test".to_string(),
                description: "Test".to_string(),
                links: vec![],
                dimensions: ContainerDimensions {
                    width: container_width,
                    depth: 100,
                    height: 100,
                    side_wing_from_box_top: 10,
                    side_wing_width: 20,
                },
            };

            // Generate SVG with test parameters
            let result = generate_svg(
                1, // rows (not relevant for width)
                columns,
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            // Calculate expected width using the formula
            let column_width = container_width + CLEARANCE_FOR_CONTAINER_WIDTH;
            let expected_width = (column_width * columns) as f32 
                + (columns + 1) as f32 * material_thickness;

            // Verify the assembled width matches the formula
            prop_assert_eq!(result.assembled_dimensions.width, expected_width);
        }
    }

    // Feature: calculate-assembled-dimensions, Property 2: Assembled Height Formula
    // **Validates: Requirements 2.1, 2.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_assembled_height_formula(
            rows in 1usize..=10,
            container_height in 50usize..=500,
            material_thickness in 1.0f32..=20.0,
        ) {
            // Create a minimal container with the generated dimensions
            let container = Container {
                vendor: "Test".to_string(),
                model: "Test".to_string(),
                description: "Test".to_string(),
                links: vec![],
                dimensions: ContainerDimensions {
                    width: 100,
                    depth: 100,
                    height: container_height,
                    side_wing_from_box_top: 10,
                    side_wing_width: 20,
                },
            };

            // Generate SVG with test parameters
            let result = generate_svg(
                rows,
                1, // columns (not relevant for height)
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            // Calculate expected height using the formula
            let expected_height = (container_height * rows) as f32 
                + material_thickness * 2.0;

            // Verify the assembled height matches the formula
            prop_assert_eq!(result.assembled_dimensions.height, expected_height);
        }
    }

    // Feature: calculate-assembled-dimensions, Property 3: Assembled Depth Equals Container Depth
    // **Validates: Requirements 3.1, 3.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_assembled_depth_equals_container_depth(
            rows in 1usize..=10,
            columns in 1usize..=10,
            container_depth in 100usize..=500,
            material_thickness in 1.0f32..=20.0,
        ) {
            // Create a minimal container with the generated dimensions
            let container = Container {
                vendor: "Test".to_string(),
                model: "Test".to_string(),
                description: "Test".to_string(),
                links: vec![],
                dimensions: ContainerDimensions {
                    width: 100,
                    depth: container_depth,
                    height: 100,
                    side_wing_from_box_top: 10,
                    side_wing_width: 20,
                },
            };

            // Generate SVG with test parameters
            let result = generate_svg(
                rows,
                columns,
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            // Verify the assembled depth equals container depth
            // regardless of rows, columns, or material thickness
            let expected_depth = container_depth as f32;
            prop_assert_eq!(result.assembled_dimensions.depth, expected_depth);
        }
    }

    // Feature: calculate-assembled-dimensions, Property 4: All Dimensions Positive
    // **Validates: Requirements 1.3, 2.3, 3.3, 4.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_all_dimensions_positive(
            rows in 1usize..=10,
            columns in 1usize..=10,
            container_width in 50usize..=500,
            container_height in 50usize..=500,
            container_depth in 100usize..=500,
            material_thickness in 1.0f32..=20.0,
        ) {
            // Create a container with all positive input dimensions
            let container = Container {
                vendor: "Test".to_string(),
                model: "Test".to_string(),
                description: "Test".to_string(),
                links: vec![],
                dimensions: ContainerDimensions {
                    width: container_width,
                    depth: container_depth,
                    height: container_height,
                    side_wing_from_box_top: 10,
                    side_wing_width: 20,
                },
            };

            // Generate SVG with test parameters
            let result = generate_svg(
                rows,
                columns,
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            // Verify all three dimensions are positive
            prop_assert!(result.assembled_dimensions.width > 0.0, 
                "Width should be positive, got: {}", result.assembled_dimensions.width);
            prop_assert!(result.assembled_dimensions.height > 0.0,
                "Height should be positive, got: {}", result.assembled_dimensions.height);
            prop_assert!(result.assembled_dimensions.depth > 0.0,
                "Depth should be positive, got: {}", result.assembled_dimensions.depth);
        }
    }

    // Feature: calculate-assembled-dimensions, Property 5: Calculation Idempotence
    // **Validates: Requirements 6.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_calculation_idempotence(
            rows in 1usize..=10,
            columns in 1usize..=10,
            container_width in 50usize..=500,
            container_height in 50usize..=500,
            container_depth in 100usize..=500,
            material_thickness in 1.0f32..=20.0,
        ) {
            // Create a container with random dimensions
            let container = Container {
                vendor: "Test".to_string(),
                model: "Test".to_string(),
                description: "Test".to_string(),
                links: vec![],
                dimensions: ContainerDimensions {
                    width: container_width,
                    depth: container_depth,
                    height: container_height,
                    side_wing_from_box_top: 10,
                    side_wing_width: 20,
                },
            };

            // Call generate_svg twice with identical inputs
            let result1 = generate_svg(
                rows,
                columns,
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            let result2 = generate_svg(
                rows,
                columns,
                material_thickness,
                &container,
                "#000000",
                "#FF0000",
            );

            // Verify both GeneratedSvg structures contain identical dimension values
            prop_assert_eq!(result1.assembled_dimensions.width, result2.assembled_dimensions.width,
                "Width should be identical across calls");
            prop_assert_eq!(result1.assembled_dimensions.height, result2.assembled_dimensions.height,
                "Height should be identical across calls");
            prop_assert_eq!(result1.assembled_dimensions.depth, result2.assembled_dimensions.depth,
                "Depth should be identical across calls");
        }
    }

    // Unit tests for specific dimension calculations
    // **Validates: Requirements 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 3.1, 3.2, 3.3**

    #[test]
    fn test_known_values_2x3_rack() {
        // Test with known values: 2×3 rack with 100mm containers and 5mm material
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 100,
                depth: 100,
                height: 100,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(2, 3, 5.0, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (100 + 4) * 3 + (3 + 1) * 5 = 312 + 20 = 332mm
        let expected_width = 332.0;
        // Height: 100 * 2 + 5 * 2 = 200 + 10 = 210mm
        let expected_height = 210.0;
        // Depth: 100mm (unchanged)
        let expected_depth = 100.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }

    #[test]
    fn test_minimum_configuration_1x1() {
        // Test minimum configuration: 1×1 rack
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 80,
                depth: 120,
                height: 60,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(1, 1, 3.0, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (80 + 4) * 1 + (1 + 1) * 3 = 84 + 6 = 90mm
        let expected_width = 90.0;
        // Height: 60 * 1 + 3 * 2 = 60 + 6 = 66mm
        let expected_height = 66.0;
        // Depth: 120mm (unchanged)
        let expected_depth = 120.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }

    #[test]
    fn test_very_small_material_thickness() {
        // Test edge case: very small material thickness
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 100,
                depth: 100,
                height: 100,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(2, 2, 0.5, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (100 + 4) * 2 + (2 + 1) * 0.5 = 208 + 1.5 = 209.5mm
        let expected_width = 209.5;
        // Height: 100 * 2 + 0.5 * 2 = 200 + 1.0 = 201mm
        let expected_height = 201.0;
        // Depth: 100mm (unchanged)
        let expected_depth = 100.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }

    #[test]
    fn test_large_configuration() {
        // Test edge case: large configuration (5×5 rack)
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 150,
                depth: 200,
                height: 120,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(5, 5, 6.0, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (150 + 4) * 5 + (5 + 1) * 6 = 770 + 36 = 806mm
        let expected_width = 806.0;
        // Height: 120 * 5 + 6 * 2 = 600 + 12 = 612mm
        let expected_height = 612.0;
        // Depth: 200mm (unchanged)
        let expected_depth = 200.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }

    #[test]
    fn test_single_row_multiple_columns() {
        // Test edge case: single row with multiple columns
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 90,
                depth: 110,
                height: 70,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(1, 4, 4.0, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (90 + 4) * 4 + (4 + 1) * 4 = 376 + 20 = 396mm
        let expected_width = 396.0;
        // Height: 70 * 1 + 4 * 2 = 70 + 8 = 78mm
        let expected_height = 78.0;
        // Depth: 110mm (unchanged)
        let expected_depth = 110.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }

    #[test]
    fn test_multiple_rows_single_column() {
        // Test edge case: multiple rows with single column
        let container = Container {
            vendor: "Test".to_string(),
            model: "Test".to_string(),
            description: "Test".to_string(),
            links: vec![],
            dimensions: ContainerDimensions {
                width: 85,
                depth: 95,
                height: 65,
                side_wing_from_box_top: 10,
                side_wing_width: 20,
            },
        };

        let result = generate_svg(4, 1, 3.5, &container, "#000000", "#FF0000");

        // Calculate expected values
        // Width: (85 + 4) * 1 + (1 + 1) * 3.5 = 89 + 7 = 96mm
        let expected_width = 96.0;
        // Height: 65 * 4 + 3.5 * 2 = 260 + 7 = 267mm
        let expected_height = 267.0;
        // Depth: 95mm (unchanged)
        let expected_depth = 95.0;

        assert_eq!(result.assembled_dimensions.width, expected_width);
        assert_eq!(result.assembled_dimensions.height, expected_height);
        assert_eq!(result.assembled_dimensions.depth, expected_depth);
    }
}
