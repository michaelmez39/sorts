use crate::lib;
use rand::prelude::*;
use sorts::*;
use plotters::prelude::*;

// fn plot_data(chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordi32, RangedCoordi32>>, col: &RGBColor, data: &Vec<usize>) 
//     -> Result<(), Box<dyn std::error::Error>> {
//     chart
//         .draw_series::<LineSeries, >(LineSeries::new(
//             data.clone(),
//             col,
//         ).into())?
//         .label("Merge Sort")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart.draw_series(PointSeries::of_element(
//         data,
//         5,
//         col,
//         &|c, s, st| {
//             return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
//             + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
//             + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 20).into_font());
//         },
//     ).into())?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {

    macro_rules! time {
        ($x:expr) => {{
            let now = std::time::Instant::now();
            $x;
            now.elapsed()
        }};
    }

    macro_rules! test {
        ($f: ident) => {{
            let mut rng = rand::thread_rng();
            let mut numbers1: Vec<usize> = (0..10000).map(|_| rng.gen::<usize>()).collect();
            let mut numbers2: Vec<usize> = (0..20000).map(|_| rng.gen::<usize>()).collect();
            let mut numbers5: Vec<usize> = (0..50000).map(|_| rng.gen::<usize>()).collect();
            let mut numbers10: Vec<usize> = (0..75000).map(|_| rng.gen::<usize>()).collect();
            vec![
                time!($f(&mut numbers1)).as_millis(),
                time!($f(&mut numbers2)).as_millis(),
                time!($f(&mut numbers5)).as_millis(),
                time!($f(&mut numbers10)).as_millis(),
            ]
        }}
    }
    // let merge_time = test!(merge_sort);
    // let heap_time = test!(heap_sort);

    // let root = BitMapBackend::new("merge_time.png", (640, 480)).into_drawing_area();
    // root.fill(&WHITE)?;
    // let mut chart = ChartBuilder::on(&root)
    //     .caption("Merge Sort Time", ("sans-serif", 50).into_font())
    //     .margin(5)
    //     .x_label_area_size(20)
    //     .y_label_area_size(20)
    //     .build_ranged(0..100000, 0..1500)?;

    // chart.configure_mesh().draw()?;
    // let merge_data = [10000, 20000, 50000, 100_000].iter().map(|x| *x as i32)
    //             .zip(merge_time.iter().map(|y| *y as i32));
                
    // let heap_data = [10000, 20000, 50000, 100_000].iter().map(|x| *x as i32)
    //             .zip(heap_time.iter().map(|y| *y as i32));

    // // let insertion_data = [10000, 20000, 50000, 100_000].iter().map(|x| *x as i32)
    // //             .zip(insertion_time.iter().map(|y| *y as i32));

    // chart
    //     .draw_series(LineSeries::new(
    //         heap_data.clone(),
    //         &GREEN,
    //     ))?
    //     .label("Heap Sort")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // // chart
    // // .draw_series(LineSeries::new(
    // //     insertion_data.clone(),
    // //     &BLUE,
    // // ))?
    // // .label("Insertion Sort")
    // // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // // chart.draw_series(PointSeries::of_element(
    // //     insertion_data,
    // //     5,
    // //     &BLUE,
    // //     &|c, s, st| {
    // //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    // //         + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    // //         + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 20).into_font());
    // //     },
    // // ))?;

    // chart
    //     .configure_series_labels()
    //     .background_style(&WHITE.mix(0.8))
    //     .border_style(&BLACK)
    //     .draw()?;
        
        let mut rng = rand::thread_rng();
        let numbers10: Vec<u8> = (0..20000).map(|_| rng.gen::<u8>()).collect();
        let mut a = numbers10.clone();
        let mut b = numbers10.clone();
        let mut c = numbers10.clone();
        println!("Single: {:.2?}", time!(bubble_sort(&mut a)));
        println!("Single Insertion: {:.2?}", time!(insertion_sort(&mut c)));
        println!("Parallel: {:.2?}", time!(odd_even_sort(&mut b)));
        // println!("Single: {:?}", a);
        // println!("Parallel: {:?}", b);
        for (i, j) in a.iter().zip(b.iter()) {
            assert_eq!(i, j);
        }
    Ok(())
}
