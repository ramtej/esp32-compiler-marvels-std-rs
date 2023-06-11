# esp32-compiler-marvels-std-rs

Compares different Rust FFT implementations on native (x86) as well as ESP32 platform with the goal to compare the results.
Rust bindings for ESP-IDF (Espressif's IoT Development Framework) are used, so 'std' code can be used for the embedded ESP platform.

The following FFT libraries are used:
- FFTW
- RustFFT / RealFFT
- Microfft


### Issues

# Expected versus Generated

TODO: Input (gen signal 3 freq)

arch[x86_64]:ffw3.amplitudes[0] = 0
arch[x86_64]:ffw3.amplitudes[1] = 0
arch[x86_64]:ffw3.amplitudes[2] = 0
arch[x86_64]:ffw3.amplitudes[3] = 8
arch[x86_64]:ffw3.amplitudes[4] = 0
arch[x86_64]:ffw3.amplitudes[5] = 0
arch[x86_64]:ffw3.amplitudes[6] = 0
arch[x86_64]:ffw3.amplitudes[7] = 0
arch[x86_64]:ffw3.amplitudes[8] = 0

arch[xtensa]:microfft.amplitudes[0] = 0
arch[xtensa]:microfft.amplitudes[1] = 2
arch[xtensa]:microfft.amplitudes[2] = 0
arch[xtensa]:microfft.amplitudes[3] = 3
arch[xtensa]:microfft.amplitudes[4] = 0
arch[xtensa]:microfft.amplitudes[5] = 5
arch[xtensa]:microfft.amplitudes[6] = 0
arch[xtensa]:microfft.amplitudes[7] = 4

arch[xtensa]:microfft.amplitudes[0] = 0
arch[xtensa]:microfft.amplitudes[1] = 2
arch[xtensa]:microfft.amplitudes[2] = 0
arch[xtensa]:microfft.amplitudes[3] = 3
arch[xtensa]:microfft.amplitudes[4] = 0
arch[xtensa]:microfft.amplitudes[5] = 5
arch[xtensa]:microfft.amplitudes[6] = 0
arch[xtensa]:microfft.amplitudes[7] = 4






