use std::fs::File;

use screech::read_wav;
use plotters::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("usage: plotwav {} [input_file [output_file [start [length]]]]", args[0]);
        return;
    }
    let input_file = args
        .get(1)
        .map(|s| s.as_ref())
        .unwrap_or("out.wav");
    let output_file = args
        .get(2)
        .map(|s| s.as_ref())
        .unwrap_or("out.png");

    let audio_buffer = read_wav(&mut File::open(input_file).unwrap()).unwrap();
    let channels = audio_buffer.metadata.channels as usize;
    let samples_per_channel = audio_buffer.data.len() / channels;

    let start = args
        .get(3)
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(0);
    let end = args
        .get(4)
        .map(|l| l.parse::<usize>().unwrap())
        .map(|l| std::cmp::min(start + l, samples_per_channel))
        .unwrap_or(samples_per_channel - start);

    let root = BitMapBackend::new(output_file, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let drawing_areas = root.split_evenly((channels, 1));
    for (area, channel) in drawing_areas.into_iter().zip(0..) {
        let mut chart = ChartBuilder::on(&area)
            .y_label_area_size(40)
            .build_cartesian_2d(start as f32..end as f32, -1.1f32..1.1f32)
            .unwrap();
        chart
            .configure_mesh()
            .draw()
            .unwrap();

        let mut data = vec![(0., 0.); end - start];
        for i in start..end {
            data[i - start] = (i as f32, audio_buffer.data[channel + i * channels]);
        }

        let series = LineSeries::new(data, &RED);
        chart.draw_series(series).unwrap();
    }
}
