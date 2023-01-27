let snd_ctx = soundio::Context::new();
snd_ctx.flush_events();

let snd_dvc = snd_ctx.defalt_input_device().expect("No input device");

