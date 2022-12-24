pub mod graph;

use graph::Graph;
use plotters::prelude::*;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
    usize::MAX,
};

fn main() {
    let n_thread =
        usize::from(thread::available_parallelism().expect("Error retrieving available cores!"));

    let color_dict: Vec<(RGBAColor, RGBAColor)> = vec![
        (RGBAColor(255, 0, 0, 0.025), RGBAColor(255, 0, 0, 1.0)),
        (RGBAColor(0, 0, 255, 0.02), RGBAColor(0, 0, 255, 1.0)),
        (RGBAColor(0, 255, 0, 0.025), RGBAColor(0, 255, 0, 1.0)),
    ];

    let mut settings = vec![200, 600, 1000];

    settings.sort();

    let root = BitMapBackend::new("best_min_cut.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).expect("Error filling the background!");

    let mut chart = ChartBuilder::on(&root)
        .caption("Min cut", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1000000f32 as f32, 0f32..100f32)
        .expect("Error while creating the chart!");

    let mut n_index = 0;
    for n in settings {
        let mut g = Graph::new(n);
        let n_iterations = n * n;
        let best_min_cut: Arc<Mutex<Vec<usize>>> =
            Arc::new(Mutex::new(Vec::with_capacity(n_iterations)));

        for _ in 0..n_iterations {
            best_min_cut
                .lock()
                .expect("Error acquiring the lock!")
                .push(MAX);
        }

        g.generate();

        let loop_function =
            |a: usize, b: usize, best_min_cut: Arc<Mutex<Vec<usize>>>, gr: Graph| {
                for i in a..b {
                    let gr = gr.clone();
                    min_cut(gr, Arc::clone(&best_min_cut), i)
                }
            };

        let blk_size = n_iterations / n_thread;
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let now = Instant::now();
        for i in 0..n_thread {
            if i < n_thread - 1 {
                let best_min_cut = Arc::clone(&best_min_cut);
                let g_copy = g.clone();
                let handle = thread::spawn(move || {
                    loop_function(blk_size * i, blk_size * (i + 1), best_min_cut, g_copy);
                });
                handles.push(handle);
            } else {
                let best_min_cut = Arc::clone(&best_min_cut);
                let g_copy = g.clone();
                let handle = thread::spawn(move || {
                    loop_function(i * blk_size, n_iterations, best_min_cut, g_copy);
                });
                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().expect("Error joining a thread!");
        }

        let elapsed_time = now.elapsed();
        println!(
            "The elapsed time for {} nodes is {:.3} s.",
            n,
            elapsed_time.as_secs_f64()
        );

        plot_best_min_cut(
            &mut chart,
            Arc::clone(&best_min_cut),
            n,
            color_dict[n_index].0,
            color_dict[n_index].1,
        )
        .expect("Error plotting the chart!");

        n_index += 1;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .expect("Error while drawing the framework!");

    root.present().expect("Error while showing the plots!");
}

pub fn min_cut(mut g: Graph, best_min_cut: Arc<Mutex<Vec<usize>>>, i_n: usize) {
    let mut n_edges = g.get_n_edges();
    let mut rng = rand::thread_rng();
    loop {
        if g.get_n_nodes() <= 2 {
            break;
        }
        if n_edges == 0 {
            best_min_cut.lock().expect("Failed acquiring the lock!")[i_n] = 0;
            return;
        }
        let random_index = rng.gen_range(0, n_edges);
        g.merge_nodes(random_index);
        g.remove_edge(random_index);
        n_edges = n_edges - 1;
    }
    best_min_cut.lock().expect("Failed acquiring the lock!")[i_n] = g.min_cut_size();
}

fn plot_best_min_cut(
    chart: &mut ChartContext<
        BitMapBackend,
        Cartesian2d<plotters::coord::types::RangedCoordf32, plotters::coord::types::RangedCoordf32>,
    >,
    best_min_cut: Arc<Mutex<Vec<usize>>>,
    n: usize,
    color_one: RGBAColor,
    color_two: RGBAColor,
) -> Result<(), Box<dyn std::error::Error>> {
    chart.configure_mesh().draw()?;

    let mut tmp: Vec<(f32, f32)> = Vec::new();
    let mut tmp_best: Vec<(f32, f32)> = Vec::new();
    let mut best: usize = MAX;
    let mut i = 0f32;
    for element in best_min_cut
        .lock()
        .expect("Error acquiring the lock!")
        .iter()
    {
        tmp.push((i, *element as f32));
        if best > *element {
            best = *element;
        }
        tmp_best.push((i, best as f32));
        i += 1f32;
    }

    chart
        .draw_series(LineSeries::new(tmp.into_iter(), &color_one))?
        .label("Min cut size for ".to_owned() + n.to_string().as_str() + " nodes")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color_one));

    chart
        .draw_series(LineSeries::new(tmp_best.into_iter(), &color_two))?
        .label("Best min cut size for ".to_owned() + n.to_string().as_str() + " nodes")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color_two));

    Ok(())
}
