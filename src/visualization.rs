use plotters::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompanyTrainingDatapoint {
    #[serde(default)]
    pub company_value: f64,
    #[serde(default)]
    pub currency: f64,
    #[serde(default)]
    pub processor_counts: Vec<usize>,
    #[serde(default)]
    pub stock: Vec<usize>,
}

#[derive(Deserialize)]
pub struct CompanyTrainingData {
    pub name: String,
    pub data: Vec<CompanyTrainingDatapoint>,
}

pub fn render_diagrams(data: Vec<CompanyTrainingData>, epoche: usize) {
    for dataset in data {
        render_diagram(dataset, &epoche.to_string()).unwrap();
    }
}

pub fn render_diagram(
    data: CompanyTrainingData,
    path_prefix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = &format!("data/{}_{}.svg", path_prefix, data.name);
    let root = SVGBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(750);

    let mut chart = ChartBuilder::on(&upper)
        .caption(
            "Company Training Performance",
            ("sans-serif", (5).percent_height()),
        )
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (0..data.data.len() as u32).,
            0..data
                .data
                .iter()
                .map(|datapoint| datapoint.company_value as u32)
                .max()
                .unwrap_or(1000000),
        );

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Data")
        .draw()?;

    let color = Palette99::pick(0).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            data.data
                .iter()
                .enumerate()
                .map(|(i, datapoint)| (i as u32, datapoint.company_value as u32)),
            color.stroke_width(3),
        ))?
        .label(data.name)
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
    Ok(())
}
