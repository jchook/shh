mod config;

use config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rodio::{Sink, Decoder, OutputStream};
use std::io::Cursor;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Embedded alert sound
const ALERT_OGG: &[u8] = include_bytes!("../assets/alert.ogg");

// Configurable constants
fn send_system_notification() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("msg")
            .arg("*")
            .arg("Please be quiet, you are too loud!")
            .output();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("display notification \"Please be quiet, you are too loud!\" with title \"Shh\"")
            .output();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("notify-send")
            .arg("Shh")
            .arg("Please be quiet, you are too loud!")
            .output();
    }
}

fn play_alert() {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create Sink");

    let cursor = Cursor::new(ALERT_OGG);
    if let Ok(source) = Decoder::new(cursor) {
        sink.append(source);
        sink.sleep_until_end();  // Wait for the sound to finish
    } else {
        eprintln!("❌ Failed to decode alert.ogg");
    }
}

fn main() {
    let config = Config::load();
    println!(
        "Threshold: {:.1}, Frequency: {:.1}, Sensitivity: {:.1}",
        config.decibel_threshold, config.alert_frequency, config.sensitivity,
    );
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let device_config = device.default_input_config().unwrap();

    let mut last_alert = Instant::now();

    let stream = device
        .build_input_stream(
            &device_config.into(),
            move |data: &[f32], _| {
                // Calculate RMS (Root Mean Square)
                let rms = data.iter().map(|s| s * s).sum::<f32>().sqrt();

                // Calculate Peak amplitude
                let peak = data.iter().cloned().fold(0.0_f32, f32::max);

                // Hybrid metric combining RMS and Peak
                let hybrid_metric = (1.0 - config.sensitivity) * rms + config.sensitivity * peak;

                // Convert hybrid metric to dB
                let db = 20.0 * hybrid_metric.max(1e-10).log10();

                // Print debug info
                if config.verbose > 0 {
                    println!(
                        "RMS: {:.5}, Peak: {:.5}, Hybrid: {:.5}, dB: {:.2}",
                        rms, peak, hybrid_metric, db
                    );
                }

                // Trigger alert if dB exceeds threshold
                if db > config.decibel_threshold && last_alert.elapsed().as_millis() > config.alert_frequency {
                    println!(
                        "Shh! RMS: {:.5}, Peak: {:.5}, Hybrid: {:.5}, dB: {:.2}",
                        rms, peak, hybrid_metric, db
                    );
                    play_alert();
                    send_system_notification();
                    last_alert = Instant::now();
                }
            },
            move |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    loop {
        sleep(Duration::from_millis(100));
    }
}
