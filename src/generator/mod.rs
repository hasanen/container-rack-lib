use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::{Document, Node};

// All measurements are in mm
const BOX_WIDTH_UNDER_LID: usize = 155;
const BOX_WDITH_WITH_LID: usize = 174;
const BOX_DEPTH: usize = 210;
const BOX_BLOCK_HEIGHT: usize = 55;
const SIDE_WING_FROM_BOX_TOP: usize = 14;
const SIDE_WING_WIDTH: usize = 8;
const EXTRA_SPACE_ON_TOP_OF_TOP_SLOT: usize = 3;
const SIDE_WING_SLOT_FROM_FRONT: usize = 20;
const SIDE_WING_SLOT_WIDTH: usize = 20;
const SIDE_WING_SLOT_SPACING: usize = 15;
const CLEARANCE_BETWEEN_TWO_WINGS_IN_PAIR: usize = 3;

pub fn generate_svg(rows: usize, columns: usize, material_thickness: usize) -> Document {
    let starting_point_x = 0;
    let starting_point_y = 0;
    let amount_of_boxes = (rows * columns) as usize;
    let height_of_two_side_wings = height_of_two_side_wings(material_thickness);
    let height_of_two_side_wings_with_clearance =
        (height_of_two_side_wings + CLEARANCE_BETWEEN_TWO_WINGS_IN_PAIR) as usize;

    let total_width = BOX_DEPTH;
    let total_height = amount_of_boxes * height_of_two_side_wings_with_clearance;

    let mut document = Document::new()
        .set("viewBox", (0, 0, total_width, total_height))
        .set("width", format!("{}mm", total_width))
        .set("height", format!("{}mm", total_height));

    for i in 0..amount_of_boxes {
        dbg!(i);
        generate_side_wings(
            &mut document,
            starting_point_x,
            starting_point_y + height_of_two_side_wings_with_clearance * i,
            material_thickness,
        );
    }

    document
}

fn top_width(columns: usize, material_thickness: usize) -> usize {
    ((material_thickness + BOX_WDITH_WITH_LID) * columns) + material_thickness
}
fn generate_side_wings(
    document: &mut Document,
    starting_point_x: usize,
    starting_point_y: usize,
    material_thickness: usize,
) {
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y,
        material_thickness,
        false,
    );
    document.append(path);
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y + SIDE_WING_WIDTH + CLEARANCE_BETWEEN_TWO_WINGS_IN_PAIR,
        material_thickness,
        true,
    );
    document.append(path);
}

fn height_of_two_side_wings(material_thickness: usize) -> usize {
    SIDE_WING_WIDTH * 2 + material_thickness + CLEARANCE_BETWEEN_TWO_WINGS_IN_PAIR
}

fn generate_side_wing(
    starting_point_x: usize,
    starting_point_y: usize,
    material_thickness: usize,
    inverted: bool,
) -> Path {
    let wing_data = if inverted {
        generate_side_wing_inverted_path(starting_point_x, starting_point_y, material_thickness)
    } else {
        generate_side_wing_path(starting_point_x, starting_point_y, material_thickness)
    };

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("d", wing_data)
}

fn generate_side_wing_path(
    starting_point_x: usize,
    starting_point_y: usize,
    material_thickness: usize,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH + material_thickness)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH)
        .horizontal_line_to(third_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH + material_thickness)
        .horizontal_line_to(third_side_wing_tap_position_from_front() + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH)
        .horizontal_line_to(BOX_DEPTH)
        .vertical_line_to(starting_point_y)
        .close()
}

fn generate_side_wing_inverted_path(
    starting_point_x: usize,
    starting_point_y: usize,
    material_thickness: usize,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y + material_thickness))
        .horizontal_line_to(second_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(second_side_wing_tap_position_from_front() + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(fourth_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(BOX_DEPTH - SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(BOX_DEPTH)
        .vertical_line_to(starting_point_y + material_thickness + SIDE_WING_WIDTH)
        .horizontal_line_to(starting_point_x)
        .close()
}

fn third_side_wing_tap_position_from_front() -> usize {
    BOX_DEPTH
        - (SIDE_WING_SLOT_FROM_FRONT
            + SIDE_WING_SLOT_WIDTH
            + SIDE_WING_SLOT_SPACING
            + SIDE_WING_SLOT_WIDTH)
}
fn second_side_wing_tap_position_from_front() -> usize {
    SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH + SIDE_WING_SLOT_SPACING
}

fn fourth_side_wing_tap_position_from_front() -> usize {
    BOX_DEPTH - (SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
}
