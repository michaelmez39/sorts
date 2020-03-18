use plotters::prelude::*;
use sorts::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Duration;

#[derive(Debug)]
struct MultiTime {
    rand: Vec<Duration>,
    ord: Vec<Duration>,
    rev: Vec<Duration>
}

macro_rules! time {
    ($x:expr, $n:expr) => {{
        (0..$n).fold(Duration::new(0, 0), |acc, _| {
            let now = std::time::Instant::now();
            $x;
            acc + now.elapsed()
        }) / $n
    }};
}
macro_rules! test_sort {
    ($f: ident) => {{
        let iters = 1;
        let mut rand10: Vec<usize> = BufReader::new(File::open("test_files/rand10000.0.txt").unwrap())
            .lines()
            .map(|c| c.unwrap().parse::<usize>().unwrap())
            .collect();
        let mut rand20: Vec<usize> = BufReader::new(File::open("test_files/rand20000.0.txt").unwrap())
            .lines()
            .map(|c| c.unwrap().parse::<usize>().unwrap())
            .collect();
        let mut rand50: Vec<usize> = BufReader::new(File::open("test_files/rand50000.0.txt").unwrap())
            .lines()
            .map(|c| c.unwrap().parse::<usize>().unwrap())
            .collect();
        let mut rand100: Vec<usize> = BufReader::new(File::open("test_files/rand100000.0.txt").unwrap())
            .lines()
            .map(|c| c.unwrap().parse::<usize>().unwrap())
            .collect();
        let rand = vec![
            time!($f(&mut rand10), iters),
            time!($f(&mut rand20), iters),
            time!($f(&mut rand50), iters),
            time!($f(&mut rand100), iters),
        ];

    let mut ord10: Vec<usize> = BufReader::new(File::open("test_files/ord10000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let mut ord20: Vec<usize> = BufReader::new(File::open("test_files/ord20000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let mut ord50: Vec<usize> = BufReader::new(File::open("test_files/ord50000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let mut ord100: Vec<usize> = BufReader::new(File::open("test_files/ord100000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let ord = vec![
        time!($f(&mut ord10), iters),
        time!($f(&mut ord20), iters),
        time!($f(&mut ord50), iters),
        time!($f(&mut ord100), iters),
    ];

    let mut rev10: Vec<usize> = BufReader::new(File::open("test_files/rev10000.0.txt").unwrap())
    .lines()
    .map(|c| c.unwrap().parse::<usize>().unwrap())
    .collect();
    let mut rev20: Vec<usize> = BufReader::new(File::open("test_files/rev20000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let mut rev50: Vec<usize> = BufReader::new(File::open("test_files/rev50000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let mut rev100: Vec<usize> = BufReader::new(File::open("test_files/rev100000.0.txt").unwrap())
        .lines()
        .map(|c| c.unwrap().parse::<usize>().unwrap())
        .collect();
    let rev = vec![
        time!($f(&mut rev10), iters),
        time!($f(&mut rev20), iters),
        time!($f(&mut rev50), iters),
        time!($f(&mut rev100), iters),
    ];

    MultiTime {
        rand,
        ord,
        rev
    }
    }};
}

#[test]
fn sort_time() {
    let results = test_sort!(selection_sort);
    let sort_name = "Selection Sort";
    let sort_path = "selection_time.png";
    let max_time = 350000;
    let x_axis = [10000, 20000, 50000, 100_000].iter().map(|x| *x);
    let ord = results.ord.iter().map(|x| x.as_millis() as i32);
    let rev = results.rev.iter().map(|x| x.as_millis() as i32);
    let rand = results.rand.iter().map(|x| x.as_millis() as i32);
    println!("results: {:?}", results);
    let root = BitMapBackend::new(sort_path, (640*2, 480*2)).into_drawing_area();
    root.fill(&WHITE).expect("can't make image file");

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} Time", sort_name), ("sans-serif", 106).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin_right(20)
        .build_ranged(0..110000, 0..max_time).expect("can't make chart widget");

    chart.configure_mesh()
        .x_label_style(("sans-serif", 30).into_font())
        .y_label_style(("sans-serif", 30).into_font())
        .x_desc("Elements (#)")
        .y_desc("Time (ms)")
        .draw().expect("can't draw mesh to chart");

    chart
        .draw_series(LineSeries::new(
            x_axis.clone().zip(ord),
            &GREEN,
        )).expect("can't draw series to chart")
        .label("Ordered")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
    .draw_series(LineSeries::new(
        x_axis.clone().zip(rev),
        &BLUE,
    )).expect("can't draw series to chart")
    .label("Reversed")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
    .draw_series(LineSeries::new(
        x_axis.zip(rand),
        &RED,
    )).expect("can't draw series to chart")
    .label("Random")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 24).into_font())
        .draw()
        .expect("couldn't draw chart to root")
}
