#[cfg(feature = "target-esp32-none-elf")]
use esp_idf_sys as _;
use num_complex::Complex32;
use std::convert::TryInto;
use std::f32::consts::PI;

#[cfg(feature = "target-native")]
use fftw::plan::*;
#[cfg(feature = "target-native")]
use fftw::types::*;
use std::env::consts::ARCH;

use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;

const X86_F32_MACHINE_EPSILON: f32 = 1.1920929e-7;

fn test_machine_epsilon() {
    let mut eps = 1.0f32;
    while eps / 2.0 + 1.0 != 1.0 {
        eps /= 2.0;
    }
    // parity with x86?
    assert_eq!(eps, X86_F32_MACHINE_EPSILON);
}

fn test_hypot() {
    let x = 2.0_f32;
    let y = 3.0_f32;

    // sqrt(x^2 + y^2)
    let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    println!(
        "arch[{}]:hypot.abs_difference = {:.prec$}",
        ARCH,
        abs_difference,
        prec = 9
    );
    assert!(abs_difference < 1e-10);
}

/// Mimics https://gitlab.com/teskje/microfft-rs/-/blame/master/src/impls/cfft.rs#L48
/// Simulates a single layer of the FFT (Fast Fourier Transform) butterfly computations,
/// and it seems the issues arise from the complex number arithmetic.
pub fn test_compute_butterflies() {
    // some arbitrary complex f32 numbers
    let mut x: Vec<Complex32> = vec![
        Complex32::new(0.0, 0.9238795),
        Complex32::new(-0.000000023849761, -0.9238794),
        Complex32::new(-1.0, -0.38268343),
        Complex32::new(1.0, 0.38268384),
    ];

    let m = 2;
    let u = m / 2;

    // [k = 0] twiddle factor: `1 + 0i`
    let (x_0, x_m) = (x[0], x[m]);
    x[0] = x_0 + x_m;
    x[m] = x_0 - x_m;

    std::hint::black_box(&x);

    // [k = m/2] twiddle factor: `0 - 1i`
    let (x_u, x_um) = (x[u], x[u + m]);
    let y = x_um * Complex32::new(0., -1.);
    println!("arch[{}]:y = {}", ARCH, y); // < here is the issue!

    x[u] = x_u + y;
    x[u + m] = x_u - y;
}

pub fn test_compute_butterflies_narrow() {
    // Some arbitrary complex f32 numbers
    let mut x: Vec<Complex32> = vec![
        Complex32::new(0.0, 0.9238795),
        Complex32::new(-0.000000023849761, -0.9238794),
        Complex32::new(-1.0, -0.38268343),
        Complex32::new(1.0, 0.38268384),
    ];

    // Twiddle factor: `1 + 0i`
    let (x_0, x_m) = (x[0], x[2]);
    x[0] = x_0 + x_m;
    x[2] = x_0 - x_m;

    // Same effect as 'println!("arch[{}]:x = {:?}", ARCH, x);'
    std::hint::black_box(&x); // <- without this, the below computation is valid?!

    // Twiddle factor: `0 - 1i`
    let y = x[1] * Complex32::new(0., -1.);
    // Note(jj): y has to wrong value, sign "flipped" in the imaginary part:
    // arch[x86_64]:y = -0.9238794+0.000000023849761i versus
    // arch[xtensa]:y = -0.9238794-0.000000023849761i
    // .. but only when the above println! or compiler hint (blackbox) is commented in!
    println!("arch[{}]:y = {}", ARCH, y);
}

#[cfg(feature = "target-native")]
fn test_ffw3() {
    let sample_count = 16;
    let signal_freq = 3.;
    let sample_interval = 1. / sample_count as f32;
    let mut samples: Vec<_> = (0..sample_count)
        .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
        .collect();

    for (i, v) in samples.iter().enumerate() {
        println!("arch[{}]:ffw3.samples[{}] = {}", ARCH, i, v);
    }
    let mut spectrum = vec![num_complex::Complex32::new(0.0, 0.0); sample_count / 2 + 1];

    let mut plan = R2CPlan32::aligned(&[sample_count], Flag::ESTIMATE).unwrap();

    plan.r2c(&mut samples, &mut spectrum).unwrap();

    // Because the input is real, the spectrum will be symmetric and the imaginary
    // part at the DC component will be zero
    spectrum[0].im = 0.0;

    // Log the spectrum
    for (i, v) in spectrum.iter().enumerate() {
        println!("arch[{}]:ffw3.spectrum[{}] = {}", ARCH, i, v);
    }

    // Compute the amplitudes
    let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm() as u32).collect();

    // Log the amplitudes
    for (i, v) in amplitudes.iter().enumerate() {
        println!("arch[{}]:ffw3.amplitudes[{}] = {}", ARCH, i, v);
    }

    assert_eq!(&amplitudes, &[0, 0, 0, 8, 0, 0, 0, 0, 0]);
}

#[allow(dead_code)]
fn test_microfft() {
    // https://github.com/SergiusIW/noisy_float-rs
    let sample_count = 16;
    let signal_freq = 3.;
    let sample_interval = 1. / sample_count as f32;
    let samples: Vec<_> = (0..sample_count)
        .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
        .collect();

    for (i, v) in samples.iter().enumerate() {
        println!("arch[{}]:microfft.samples[{}] = {}", ARCH, i, v);
    }

    // compute the RFFT of the samples
    let mut samples: [_; 16] = samples.try_into().unwrap();
    let spectrum = microfft::real::rfft_16(&mut samples);
    spectrum[0].im = 0.0;

    for (i, v) in spectrum.iter().enumerate() {
        println!("arch[{}]:microfft.spectrum[{}] = {}", ARCH, i, v);
    }

    // the spectrum has a spike at index `signal_freq`
    let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm() as u32).collect();
    for (i, v) in amplitudes.iter().enumerate() {
        println!("arch[{}]:microfft.amplitudes[{}] = {}", ARCH, i, v);
    }
    // Note(jj): Fails on esp32 but works on x86
    //assert_eq!(&amplitudes, &[0, 0, 0, 8, 0, 0, 0, 0]);
}

#[allow(dead_code)]
fn test_realfft() {
    let sample_count = 16;
    let signal_freq = 3.;
    let sample_interval = 1. / sample_count as f32;
    let samples: Vec<_> = (0..sample_count)
        .map(|i| (2. * PI * signal_freq * sample_interval * i as f32).sin())
        .collect();

    for (i, v) in samples.iter().enumerate() {
        println!("arch[{}]:realfft.samples[{}] = {}", ARCH, i, v);
    }

    let mut samples: [_; 16] = samples.try_into().unwrap();

    let mut fft = RealFftPlanner::<f32>::new();
    let plan = fft.plan_fft_forward(samples.len());

    let mut spectrum = Vec::new();
    for _i in 0..(samples.len() / 2 + 1) {
        spectrum.push(Complex::new(0.0f32, 0.0f32))
    }

    plan.process(&mut samples, &mut spectrum).unwrap();
    spectrum[0].im = 0.0;

    for (i, v) in spectrum.iter().enumerate() {
        println!("arch[{}]:realfft.spectrum[{}] = {}", ARCH, i, v);
    }

    let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm() as u32).collect();
    for (i, v) in amplitudes.iter().enumerate() {
        println!("arch[{}]:realfft.amplitudes[{}] = {}", ARCH, i, v);
    }
    // Note(jj): Fails on esp32 but works on x86
    //assert_eq!(&amplitudes, &[0, 0, 0, 8, 0, 0, 0, 0, 0]);
}

fn main() {
    #[cfg(feature = "target-esp32-none-elf")]
    {
        esp_idf_sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();
    }
    // Some f32 floats warm up the FPU
    test_machine_epsilon();
    test_hypot();

    #[cfg(feature = "target-native")]
    {
        println!("Execute FFTW reference on native (e.g. x86) target");
        test_ffw3();
    }
    //test_compute_butterflies();
    test_compute_butterflies_narrow();

    //test_microfft();
    //test_realfft();

    loop {}
}
