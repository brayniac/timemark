#![feature(test)]
#![feature(asm)]

#[macro_use]
extern crate log;

extern crate allan;
extern crate pad;
extern crate test;
extern crate time;

const ONE_MILISECOND: u64 = 1_000_000;
const ONE_SECOND: u64 = 1_000 * ONE_MILISECOND;

use pad::{PadStr, Alignment};
use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord};

use std::time::Duration;

use allan::Allan;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let ms = format!(
                "{:.*}",
                3,
                ((time::precise_time_ns() % ONE_SECOND) / ONE_MILISECOND)
            );
            println!(
                "{}.{} {:<5} [{}] {}",
                time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
                ms.pad(3, '0', Alignment::Right, true),
                record.level().to_string(),
                "timemark",
                record.args()
            );
        }
    }
}

fn set_log_level(level: usize) {
    let log_filter;
    match level {
        0 => {
            log_filter = LogLevelFilter::Info;
        }
        1 => {
            log_filter = LogLevelFilter::Debug;
        }
        _ => {
            log_filter = LogLevelFilter::Trace;
        }
    }
    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(log_filter);
        Box::new(SimpleLogger)
    });
}

#[cfg(not(feature = "amd"))]
#[allow(unused_mut)]
pub fn rdtsc() -> u64 {
    let mut l: u32;
    let mut m: u32;
    unsafe {
        asm!("lfence; rdtsc" : "={eax}" (l), "={edx}" (m) ::: "volatile");
    }
    ((m as u64) << 32) | (l as u64)
}

#[cfg(feature = "amd")]
#[allow(unused_mut)]
pub fn rdtsc() -> u64 {
    let mut l: u32;
    let mut m: u32;
    unsafe {
        asm!("mfence; rdtsc" : "={eax}" (l), "={edx}" (m) ::: "volatile");
    }
    ((m as u64) << 32) | (l as u64)
}

fn main() {
    set_log_level(0);

    let mut allan = Allan::configure()
        .style(allan::Style::DecadeDeci)
        .max_tau(1_000_000)
        .build()
        .unwrap();

    info!("Calibrating tsc");
    let _ = tsc_ghz(Duration::new(1, 0)); // warmup
    let ghz = tsc_ghz(Duration::new(5, 0)); // real reading
    info!("ghz: {}", ghz);

    let t0 = time::precise_time_ns();
    let a0 = rdtsc();

    let mut t1 = t0 + ONE_SECOND;

    loop {
        let t = time::precise_time_ns();
        if t >= t1 {
            let a1 = rdtsc(); // get tsc
            let t2 = (a1 - a0) as f64 / ghz + t0 as f64;
            let p = (t2 - t as f64) / ONE_SECOND as f64;

            info!("a1: {} t0: {} t1: {} t2: {}, p: {}", a1, t0, t1, t2, p);

            allan.record(p);

            for i in 0..6 {
                for j in 1..10 {
                    let tau = 10_u64.pow(i) * j;
                    let adev = allan.get(tau as usize).unwrap().deviation().unwrap_or(0.0);
                    info!("{:e} T={}", adev, tau);
                }
            }
            t1 += ONE_SECOND;
        }
    }
}

fn tsc_ghz(duration: Duration) -> f64 {
    let time_0 = time::precise_time_ns();
    let tsc_0 = rdtsc();

    let time_1 = time_0 + duration.as_secs() * ONE_SECOND;

    let mut t = time::precise_time_ns();
    loop {
        if t >= time_1 {
            break;
        }
        t = time::precise_time_ns();
    }

    let tsc = rdtsc();

    let time_d = t - time_0;
    let tsc_d = tsc - tsc_0;


    (tsc_d) as f64 / time_d as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use time;
    use std::time::Instant;

    #[bench]
    fn bench_precise_time_ns(b: &mut Bencher) {
        b.iter(|| time::precise_time_ns());
    }

    #[bench]
    fn bench_time_instant(b: &mut Bencher) {
        b.iter(|| Instant::now());
    }

    #[bench]
    fn bench_ns_sub(b: &mut Bencher) {
        b.iter(|| {
            let t0 = time::precise_time_ns();
            let t1 = time::precise_time_ns();
            t1 - t0
        });
    }

    #[bench]
    fn bench_instant_sub(b: &mut Bencher) {
        b.iter(|| {
            let t0 = Instant::now();
            let t1 = Instant::now();
            t1 - t0
        });
    }

    #[bench]
    fn bench_rdtsc(b: &mut Bencher) {
        b.iter(|| rdtsc());
    }
}
