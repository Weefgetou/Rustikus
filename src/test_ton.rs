use rodio::Sink;


fn main(){
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    // Add a dummy source of the sake of the example.
    let source = rodio::source::SineWave::new(440);
    sink.append(source);
}
