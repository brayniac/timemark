# timemark - time experiments with rust

timemark is a simple rust binary which is used to demonstrate different methods of reading the current time. Currently, we support the Rust std::time::Instant, time::get_precise_time_ns(), and using x86_64 assembly to read the Time Stamp Counter from userspace. We explore the performance differences, and attempt to quantify stability of the system clock source clock_gettime() vs the CPU TSC.

## Getting timemark

timemark is built through the `cargo` command which ships with rust. Follow the instructions on [rust-lang.org][2] to get nightly rust and cargo installed. timemark uses features only enabled in nightly rust, so this is a requirement for using timemark

### Benchmark Mode

With nightly rust installed:

```shell
git clone https://github.com/brayniac/timemark.git
cd timemark
cargo bench
```

### Clocksource stability mode

With nightly rust installed:

```shell
git clone https://github.com/brayniac/timemark.git
cd timemark
cargo build --release
./target/release/timemark
```

This will produce a binary at `./target/release/timemark` which can be run in-place or copied to a more convenient location on your system.

## Sample Benchmark Output

```
test tests::bench_instant_sub     ... bench:          60 ns/iter (+/- 2)
test tests::bench_ns_sub          ... bench:          39 ns/iter (+/- 1)
test tests::bench_precise_time_ns ... bench:          20 ns/iter (+/- 1)
test tests::bench_rdtsc           ... bench:           8 ns/iter (+/- 0)
test tests::bench_time_instant    ... bench:          26 ns/iter (+/- 1)
```

Here we can see the added expense of working with std::time::Instant. Both subtraction of the `Instant` as well as reading it are longer than time::precise_time_ns(). Reading the CPU TSC is much faster than either method.

## Sample Stability Output
```
2016-08-18 04:55:57.857 INFO  [timemark] Calibrating tsc
2016-08-18 04:56:03.857 INFO  [timemark] ghz: 2.3944601487744035
...
2016-08-18 04:57:17.513 INFO  [timemark] a1: 1853353109270973 t0: 780735513491551 t1: 780776513491551 t2: 780776513491473, p: -0.000000094
2016-08-18 04:57:17.513 INFO  [timemark] 2.570646922656151e-7 T=1
2016-08-18 04:57:17.513 INFO  [timemark] 1.341013203338698e-7 T=2
2016-08-18 04:57:17.513 INFO  [timemark] 7.284239125189157e-8 T=3
2016-08-18 04:57:17.513 INFO  [timemark] 6.790716101098594e-8 T=4
2016-08-18 04:57:17.513 INFO  [timemark] 5.561773502756809e-8 T=5
2016-08-18 04:57:17.513 INFO  [timemark] 4.560249315294994e-8 T=6
2016-08-18 04:57:17.513 INFO  [timemark] 4.0283593855435426e-8 T=7
2016-08-18 04:57:17.513 INFO  [timemark] 3.442763619229561e-8 T=8
2016-08-18 04:57:17.513 INFO  [timemark] 3.113557674899815e-8 T=9
2016-08-18 04:57:17.513 INFO  [timemark] 3.0682080612053086e-8 T=10
2016-08-18 04:57:17.513 INFO  [timemark] 5.316559111046354e-9 T=20
...
```

Here we see the calculated TSC frequency, and each second, stats are reported. We calculate the overlapping [Allan Deviation][3] of the calculated phase difference between the system clocksource and the CPU TSC. This is an attempt to quantify the stability of the clocksource and TSC relative to eachother.

[1]: https://github.com/brson/multirust
[2]: https://rust-lang.org/
[3]: https://en.wikipedia.org/wiki/Allan_variance