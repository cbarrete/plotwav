use std::fs::File;

use screech::read_wav;
use plotters::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("usage: plotwav {} [input_file [output_file]]", args[0]);
        return;
    }
    let input_file = args.get(1).map(|s| s.as_ref()).unwrap_or("out.wav");
    let output_file = args.get(2).map(|s| s.as_ref()).unwrap_or("out.png");

    let audio_buffer = read_wav(&mut File::open(input_file).unwrap()).unwrap();

    let root = BitMapBackend::new(output_file, (1000, 1000)).into_drawing_area();
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
