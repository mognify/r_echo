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
