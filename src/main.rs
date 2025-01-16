use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rodio::{Decoder, OutputStream, source::Source};
use std::io::Cursor;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Embedded alert sound
const ALERT_WAV: &[u8] = include_bytes!("alert.wav");

// Configurable constants
const DECIBEL_THRESHOLD: f32 = -30.0;  // Sensitivity in dB
const ALERT_FREQUENCY: u64 = 1;        // Time between alerts in seconds
const SENSITIVITY: f32 = 0.8;          // 0.0 (RMS) to 1.0 (Peak)

fn send_system_notification() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("msg").arg("*").arg("Please be quiet, you are too loud!").output();
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
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(ALERT_WAV);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device available");
    let config = device.default_input_config().unwrap();

    let mut last_alert = Instant::now();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _| {
            // Calculate RMS (Root Mean Square)
            let rms = data.iter().map(|s| s * s).sum::<f32>().sqrt();

            // Calculate Peak amplitude
            let peak = data.iter().cloned().fold(0.0_f32, f32::max);

            // Hybrid metric combining RMS and Peak
            let hybrid_metric = (1.0 - SENSITIVITY) * rms + SENSITIVITY * peak;

            // Convert hybrid metric to dB
            let db = 20.0 * hybrid_metric.max(1e-10).log10();

            // Print debug info
            println!(
                "RMS: {:.5}, Peak: {:.5}, Hybrid: {:.5}, dB: {:.2}",
                rms, peak, hybrid_metric, db
            );

            // Trigger alert if dB exceeds threshold
            if db > DECIBEL_THRESHOLD && last_alert.elapsed().as_secs() > ALERT_FREQUENCY {
                play_alert();
                send_system_notification();
                last_alert = Instant::now();
            }
        },
        move |err| eprintln!("Stream error: {}", err),
        None,
    ).unwrap();

    stream.play().unwrap();

    loop {
        sleep(Duration::from_millis(100));
    }
}
