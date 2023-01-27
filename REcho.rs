// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

/*
to start:
get mic
get speakers
play mic through speakers at great delay

next:
recognize voice
store recognized voice portion
play RVP later
*/

//https://docs.rs/soundio/latest/soundio/
let snd_ctx = soundio::Context::new();
snd_ctx.flush_events();

let snd_dvc = snd_ctx.defalt_input_device().expect("No input device");

if !snd_dvc.supports_layout(soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo)) {
    return Err("Device doesn't support stereo".to_string());
}
if !snd_dvc.supports_format(soundio::Format::S16LE) {
    return Err("Device doesn't support S16LE".to_string());
}
if !snd_dvc.supports_sample_rate(44100) {
    return Err("Device doesn't 44.1 kHz".to_string());
}
//
fn read_callback(stream: &mut soundio::InStreamReader) {
    let frame_count_max = stream.frame_count_max();
    if let Err(e) = stream.begin_read(frame_count_max) {
        println!("Error reading from stream: {}", e);
        return;
    }
    
    for f in 0..stream.frame_count() {
        for c in 0..stream.channel_count() {
            store_audio(stream.sample::<i16>(c, f));
        }
    }
}

let mut input_stream = snd_dvc.open_instream(
    44100,
    soundio::Format::S16LE,
    soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo),
    2.0,
    read_callback,
    None::<fn()>,
    None::<fn(soundio::Error)>,
)?;
input_stream.start()?;
