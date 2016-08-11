#![feature(test)]
#![feature(asm)]

#[macro_use]
extern crate log;

extern crate test;
extern crate time;
extern crate shuteye;
extern crate pad;

extern crate allan;

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
            let ms = format!("{:.*}",
                             3,
                             ((time::precise_time_ns() % 1_000_000_000) / 1_000_000));
            println!("{}.{} {:<5} [{}] {}",
                     time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
                     ms.pad(3, '0', Alignment::Right, true),
                     record.level().to_string(),
                     "timemark",
                     record.args());
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


fn main() {
	set_log_level(0);

	let mut allan = Allan::new();

    loop {
    	let time_0 = time::precise_time_ns();
	    let tsc_0 = rdtsc();

	    shuteye::sleep(Duration::new(1,0));

	    let time_1 = time::precise_time_ns();
	    let tsc_1 = rdtsc();

	    let time_d = time_1 - time_0;
	    let tsc_d = tsc_0 - tsc_1;

	    let frequency = tsc_d as f64 / time_d as f64/ 18444818948_f64;

	    info!("{} Hz", frequency);
	    allan.record(frequency as f64);

	    info!("ADEV: (   t=1) {}", allan.get(1).unwrap().deviation().unwrap_or(0.0));
	    info!("ADEV: (  t=10) {}", allan.get(10).unwrap().deviation().unwrap_or(0.0));
	    info!("ADEV: ( t=100) {}", allan.get(100).unwrap().deviation().unwrap_or(0.0));
	    info!("ADEV: (t=1000) {}", allan.get(1000).unwrap().deviation().unwrap_or(0.0));
    }
}

#[allow(unused_mut)]
pub fn rdtsc() -> u64 {
	unsafe {
		let mut low: u32;
	    let mut high: u32;

	    asm!("rdtsc" : "={eax}" (low), "={edx}" (high));
	    ((high as u64) << 32) | (low as u64)
	} 
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use time;
    use std::time::Instant;

    #[bench]
    fn bench_precise_time_ns(b: &mut Bencher) {
        b.iter(|| {
        	time::precise_time_ns()
        });
    }

    #[bench]
    fn bench_time_instant(b: &mut Bencher) {
        b.iter(|| {
        	Instant::now()
        });
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
    	b.iter(|| {
    		rdtsc()
    	});
    }
}