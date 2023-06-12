# esp32-compiler-marvels-std-rs

Compares various Rust FFT implementations on native (x86) as well as ESP32 (xtensa/esp32s3) platform with the goal to compare the results.
Rust bindings for ESP-IDF (Espressif's IoT Development Framework) are used, so 'std' Rust code is used for the embedded ESP platform.

The following FFT libraries are used:
- FFTW
- RustFFT / RealFFT
- Microfft


## Issues

### Expected versus Generated

**Input**: A simple sinusoidal time series is generated which is small enough to fit into the RAM of the ESP32 platform.

```
    let sample_count = 16;
    let signal_freq = 3.;
    let sample_interval = 1. / sample_count as f32;
    let mut samples: Vec<_> = (0..sample_count)
        .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
        .collect();
```

**Output**: A forward FFT operation is performed on the time series and a normalized amplitude of the spectrum is calculated - for x86 and ESP32 as well as for the different FFT libraries. FFTW is only executed on x86 and serves as a reference.

The expected result is as follows:

```
arch[x86_64]:ffw3.amplitudes[0] = 0
arch[x86_64]:ffw3.amplitudes[1] = 0
arch[x86_64]:ffw3.amplitudes[2] = 0
arch[x86_64]:ffw3.amplitudes[3] = 8 <- Sigle peak
arch[x86_64]:ffw3.amplitudes[4] = 0
arch[x86_64]:ffw3.amplitudes[5] = 0
arch[x86_64]:ffw3.amplitudes[6] = 0
arch[x86_64]:ffw3.amplitudes[7] = 0
arch[x86_64]:ffw3.amplitudes[8] = 0
```

However, depending on the compiler optimization level, the following results are produced. This is only true for the ESP32 platform (xtensa), on the x86 all compiler optimization levels produce correct results.

opt-level = 0

```
arch[xtensa]:microfft.amplitudes[0] = 0
arch[xtensa]:microfft.amplitudes[1] = 0
arch[xtensa]:microfft.amplitudes[2] = 0
arch[xtensa]:microfft.amplitudes[3] = 8
arch[xtensa]:microfft.amplitudes[4] = 0
arch[xtensa]:microfft.amplitudes[5] = 1 <- Wrong
arch[xtensa]:microfft.amplitudes[6] = 0
arch[xtensa]:microfft.amplitudes[7] = 0
..
..
arch[xtensa]:realfft.amplitudes[0] = 0
arch[xtensa]:realfft.amplitudes[1] = 0
arch[xtensa]:realfft.amplitudes[2] = 0
arch[xtensa]:realfft.amplitudes[3] = 8 <- Valid
arch[xtensa]:realfft.amplitudes[4] = 0
arch[xtensa]:realfft.amplitudes[5] = 0
arch[xtensa]:realfft.amplitudes[6] = 0
arch[xtensa]:realfft.amplitudes[7] = 0
arch[xtensa]:realfft.amplitudes[8] = 0

```

opt-level = 1

```
arch[xtensa]:microfft.amplitudes[0] = 0
arch[xtensa]:microfft.amplitudes[1] = 2 <- ?
arch[xtensa]:microfft.amplitudes[2] = 0
arch[xtensa]:microfft.amplitudes[3] = 3 <- ?
arch[xtensa]:microfft.amplitudes[4] = 0
arch[xtensa]:microfft.amplitudes[5] = 5 <- ?
arch[xtensa]:microfft.amplitudes[6] = 0
arch[xtensa]:microfft.amplitudes[7] = 4 <- ?
..
..
arch[xtensa]:realfft.amplitudes[0] = 0
arch[xtensa]:realfft.amplitudes[1] = 0
arch[xtensa]:realfft.amplitudes[2] = 0
arch[xtensa]:realfft.amplitudes[3] = 0
arch[xtensa]:realfft.amplitudes[4] = 0
arch[xtensa]:realfft.amplitudes[5] = 8 <- Valid value, but shifted in frequency!?
arch[xtensa]:realfft.amplitudes[6] = 0
arch[xtensa]:realfft.amplitudes[7] = 0
arch[xtensa]:realfft.amplitudes[8] = 0
```

opt-level = 3

```
arch[xtensa]:microfft.amplitudes[0] = 0
arch[xtensa]:microfft.amplitudes[1] = 2 <- ?
arch[xtensa]:microfft.amplitudes[2] = 0
arch[xtensa]:microfft.amplitudes[3] = 3 <- ?
arch[xtensa]:microfft.amplitudes[4] = 0
arch[xtensa]:microfft.amplitudes[5] = 5 <- ?
arch[xtensa]:microfft.amplitudes[6] = 0
arch[xtensa]:microfft.amplitudes[7] = 4 <- ?
..
..
arch[xtensa]:realfft.amplitudes[0] = 0
arch[xtensa]:realfft.amplitudes[1] = 0
arch[xtensa]:realfft.amplitudes[2] = 0
arch[xtensa]:realfft.amplitudes[3] = 0
arch[xtensa]:realfft.amplitudes[4] = 0
arch[xtensa]:realfft.amplitudes[5] = 8 <- Valid value, but shifted in frequency!?
arch[xtensa]:realfft.amplitudes[6] = 0
arch[xtensa]:realfft.amplitudes[7] = 0
arch[xtensa]:realfft.amplitudes[8] = 0
```



