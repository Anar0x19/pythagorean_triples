use plotters::prelude::*;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let limit = 100;
    let mut triples = Vec::new();

    for a in 1..=limit {
        for b in a..=limit {
            let c_squared = a * a + b * b;
            let c = (c_squared as f64).sqrt() as i32;
            if c * c == c_squared && c <= limit {
                triples.push((a, b, c));
            }
        }
    }

    let mut file = File::create
    ("PT.txt")?;
    for (i, (a, b, c)) in triples.iter().enumerate() {
        writeln!(file, "x{}: ({}, {}, {})", i + 1, a, b, c)?;
    }

    let root = BitMapBackend::new("PT.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Pythagorean Triples", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..limit, 0..limit)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (i, (a, b, _c)) in triples.iter().enumerate() {
        chart
            .draw_series(PointSeries::of_element(
                [(*a, *b)],
                6,
                &RED,
                &|coord, size, style| {
                    return EmptyElement::at(coord)
                        + Circle::new((1, 0), size, style.filled())
                        + Text::new(
                            format!("x{}", i + 2),
                            (11, -10),
                            ("sans-serif", 16).into_font(),
                        );
                },
            ))
            .unwrap();
    }

    println!("Found {}", triples.len());

    Ok(())
}

