use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;
use std::time::Duration;
use std::thread;


pub fn keep_speaker_awake() {

    // Get the default output stream
    println!("JIGGLER: WILL WAIT FOR AUDIO DEVICE");
    let x;
    loop{
        if let Ok(os) = OutputStream::try_default(){
            x = os;
            break
        }else{
            // wait a while and try again?
            println!("Failed to open output stream. Will try again in 5 seconds.");
            thread::sleep(Duration::from_secs(5));
        }
    }
    let (_stream, stream_handle) = x;

    println!("JIGGLER: GOT AUDIO DEVICE");

    // Wait for bluetooth speaker to be ready
    thread::sleep(Duration::from_secs(5));
    
    println!("JIGGLER: BOOP BOOP BOOP!");
    // Startup Boop
    boop(&stream_handle);
    thread::sleep(Duration::from_secs_f32(0.1));
    boop(&stream_handle);
    thread::sleep(Duration::from_secs_f32(0.1));
    boop(&stream_handle);
    thread::sleep(Duration::from_secs_f32(0.1));

    loop {
        
        thread::sleep(Duration::from_secs(120));

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

    }
}


/// Notify the user that the system has started up as intended
fn boop(stream_handle:&OutputStreamHandle){
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(300.0)
        .take_duration(Duration::from_secs_f32(0.2))
        .amplify(0.1);
    sink.append(source);
    sink.sleep_until_end();
}
