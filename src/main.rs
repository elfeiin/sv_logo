use std::ops::{Add, Mul, Sub};
use ultraviolet::{Rotor2, Vec2};

use svg::node::element::path::{Command, Data};
use svg::node::element::{
    Circle, Definitions, LinearGradient, Path, RadialGradient, Rectangle, Stop,
};
use svg::Document;

fn make_star(
    center: Vec2,
    outer_radius: f32,
    inner_radius_multiplier: f32,
    spokes: u32,
    color: &str,
) -> Path {
    let angle = -std::f32::consts::TAU / spokes as f32;
    let outer_pos_start = Vec2::new(0.0, outer_radius);
    let inner_radius = {
        let pos_0 = Rotor2::from_angle(angle) * outer_pos_start;
        let pos_1 = Rotor2::from_angle(angle * (spokes - 1) as f32 / -2.0) * outer_pos_start;
        let diff = pos_0 - pos_1;
        let slope = diff.y / diff.x;
        pos_0.y - slope * pos_0.x
    };
    let inner_pos_start = Vec2::new(0.0, inner_radius * inner_radius_multiplier);
    let mut points = Vec::with_capacity(10);

    for i in 0..spokes {
        let rotor = Rotor2::from_angle(angle * i as f32);
        points.push(rotor * -outer_pos_start + center);
        let rotor = Rotor2::from_angle(angle * i as f32 + angle / 2.0);
        points.push(rotor * inner_pos_start + center);
    }

    let mut data = Data::new();
    if let Some(point) = points.get(0) {
        data = data.move_to((point.x, point.y));
    }
    for point in points.iter().skip(1) {
        data = data.line_to((point.x, point.y));
    }
    data = data.close();

    Path::new()
        .set("fill", color)
        .set("stroke", "none")
        .set("d", data)
}

fn make_segmented_star(
    center: Vec2,
    outer_radius: f32,
    inner_radius_multiplier: f32,
    spokes: usize,
    colors: Vec<&str>,
) -> Vec<Path> {
    let angle = -std::f32::consts::TAU / spokes as f32;
    let outer_pos_start = Vec2::new(0.0, outer_radius);
    let inner_radius = inner_radius_multiplier * {
        let pos_0 = Rotor2::from_angle(angle) * outer_pos_start;
        let pos_1 = Rotor2::from_angle(angle * (spokes - 1) as f32 / -2.0) * outer_pos_start;
        let diff = pos_0 - pos_1;
        let slope = diff.y / diff.x;
        pos_0.y - slope * pos_0.x
    };
    let inner_pos_start = Vec2::new(0.0, inner_radius);
    let mut points = Vec::with_capacity(spokes * 2);

    for i in 0..spokes {
        let rotor = Rotor2::from_angle(angle * i as f32);
        points.push(rotor * -outer_pos_start + center);
        let rotor = Rotor2::from_angle(angle * i as f32 + angle / 2.0);
        points.push(rotor * inner_pos_start + center);
    }

    let mut triangles: Vec<[Vec2; 2]> = Vec::with_capacity(spokes * 2);

    if let Some(point) = points.get(0) {
        let mut previous = point;
        for (i, point) in points.iter().skip(1).enumerate() {
            if i % 2 == 1 {
                triangles.push([*point, *previous]);
            } else {
                triangles.push([*previous, *point]);
            }
            previous = point;
        }
        triangles.push([*point, *previous]);
    }

    let mut output = Vec::with_capacity(spokes * 2);
    for (i, [v0, v1]) in triangles.iter().enumerate() {
        let data = Data::new()
            .move_to((v0.x, v0.y))
            .line_to((v1.x, v1.y))
            .line_to((center.x, center.y))
            .close();
        output.push(
            Path::new()
                .set("fill", *colors.get(i % colors.len()).unwrap_or(&"#ffffff"))
                .set("stroke", "none")
                .set("d", data),
        );
    }
    output
}

const WIDTH: f32 = 160.0;
const HEIGHT: f32 = 160.0;
const SCALE: f32 = 8.0;

const MAIN_SIZE: f32 = 45.0;

const BG_COLOR: &str = "#002a96";

fn main() {
    let defs = Definitions::new()
        .add(
            LinearGradient::new()
                .set("x1", "0")
                .set("x2", "0")
                .set("y1", "0")
                .set("y2", "1")
                .set("id", "gradient1")
                .add(Stop::new().set("stop-color", "#dcb37e").set("offset", "0%"))
                .add(
                    Stop::new()
                        .set("stop-color", "#fefac9")
                        .set("offset", "50%"),
                ),
        )
        .add(
            LinearGradient::new()
                .set("x1", 0)
                .set("x2", (std::f32::consts::TAU / 10.0).sin())
                .set("y1", 0)
                .set("y2", (std::f32::consts::TAU / 10.0).cos())
                .set("id", "gradient2")
                .add(
                    Stop::new()
                        .set("stop-color", "#ffffff")
                        .set("offset", "50%"),
                )
                .add(
                    Stop::new()
                        .set("stop-color", "#dcdad7")
                        .set("offset", "100%"),
                ),
        );
    let pink_circle = Circle::new()
        .set("fill", "#f3a9db")
        .set("cx", 0.0)
        .set("cy", 0.0)
        .set("r", MAIN_SIZE * 1.2);

    let white_circle = Circle::new()
        .set("fill", "url(#gradient2)")
        .set("cx", 0.0)
        .set("cy", 0.0)
        .set("r", MAIN_SIZE);

    let star_triangles = make_segmented_star(
        Vec2::new(0.0, 0.0),
        MAIN_SIZE,
        1.5,
        5,
        vec![
            "#fdffe7",
            "#fcfdeb",
            "#f8d28f",
            "#fefde8",
            "#f3d090",
            "#c89262",
            "#f5ce8d",
            "#c49963",
            "url(#gradient1)",
            "#f8cf90",
        ],
    );

    let rect = Rectangle::new()
        .set("fill", BG_COLOR)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("x", WIDTH / -2.0)
        .set("y", HEIGHT / -2.0);

    let mut document = Document::new()
        .set("viewBox", (-WIDTH / 2.0, -HEIGHT / 2.0, WIDTH, HEIGHT))
        .set("width", WIDTH * SCALE)
        .set("height", HEIGHT * SCALE)
        .add(defs)
        .add(rect)
        .add(pink_circle)
        .add(white_circle);

    for triangle in star_triangles {
        document = document.add(triangle);
    }

    svg::save("image.svg", &document).unwrap();
}
