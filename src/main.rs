use std::fs::File;

use screech::read_wav;
use plotters::prelude::*;

fn main() {
    let audio_buffer = read_wav(&mut File::open("out.wav").unwrap()).unwrap();

    let root = BitMapBackend::new("out.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..audio_buffer.data.len() as f32, -1.1f32..1.1f32)
        .unwrap();
    chart
        .configure_mesh()
        .draw()
        .unwrap();

    let data = audio_buffer
        .data
        .iter()
        .enumerate()
        .map(|(x, y)| (x as f32, y.clone()))
        .collect::<Vec<(f32, f32)>>();
    let series = LineSeries::new(data, &RED);
    chart.draw_series(series).unwrap();
}
