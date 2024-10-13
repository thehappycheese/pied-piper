use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;
use std::time::Duration;
use std::thread;


pub fn keep_speaker_awake() {

    // Get the default output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        // Create a sink
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Create white noise source
        let source = SineWave::new(50.0);

        // Take the first samples corresponding to one second
        let one_second = source.take_duration(Duration::from_secs(2));

        // Adjust volume to be very low
        let low_volume = one_second.amplify(0.01);  // Adjust the volume as needed

        // Play the white noise
        sink.append(low_volume);

        // Wait for the sink to finish
        sink.sleep_until_end();

        thread::sleep(Duration::from_secs(120));
    }
}