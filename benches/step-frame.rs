extern crate criterion;
extern crate nes_rust;

use criterion::Criterion;
use nes_rust::{
    default_audio::DefaultAudio, default_display::DefaultDisplay, default_input::DefaultInput,
    rom::Rom, Nes,
};

fn bench(bench: &mut Criterion) {
    bench.bench_function("step-frame", |b| {
        let mut nes = Nes::new(
            Box::new(DefaultInput::new()),
            Box::new(DefaultDisplay::new()),
            Box::new(DefaultAudio::new()),
        );
        nes.set_rom(Rom::new(include_bytes!("data/tetris.nes").to_vec()));
        nes.bootup();
        b.iter(|| {
            for _ in 0..10 {
                nes.step_frame();
            }
        })
    });
}

pub fn benches() {
    #[cfg(not(feature = "smol_bench"))]
    let mut criterion: criterion::Criterion<_> = Criterion::default()
        .sample_size(2000)
        .significance_level(0.1);
    #[cfg(feature = "smol_bench")]
    let mut criterion: criterion::Criterion<_> = {
        use std::time::Duration;
        Criterion::default()
            .measurement_time(Duration::from_millis(1))
            .sample_size(10)
            .nresamples(1)
            .without_plots()
            .configure_from_args()
    };
    bench(&mut criterion);
}

fn main() {
    benches();
    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
