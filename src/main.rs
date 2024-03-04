use std::collections::HashMap;
use std::env::args;
use std::time::Duration;
use rodio::{source, OutputStream, Sink};
use rodio::source::{Amplify, SineWave, Source, TakeDuration};

#[derive(Clone)]
struct Pitch {
    freq: f32,
    nanos: u64,
    amp: f32,
}

impl Pitch {
    fn new(freq: f32, nanos: u64, amp: f32) -> Self {
        Pitch{
            freq,
            nanos,
            amp
        }
    }
    fn to_source(&self) -> Amplify<TakeDuration<SineWave>> {
        let p = self;
        let dur = Duration::from_nanos(p.nanos);
        SineWave::new(p.freq).take_duration(dur).amplify(p.amp)
    }
}

struct PhoneKeys {
    freq_map: HashMap<char,(f32,f32)>,
}

impl PhoneKeys {
    fn new() -> Self {
        const FV: [f32; 4] = [697.0, 770.0, 852.0, 941.0];
        const FH: [f32; 4] = [1209.0, 1336.0, 1477.0, 1633.0];
        const CHARS: [char; 16] = [
        '1', '2', '3', 'A',
        '4', '5', '6', 'B',
        '7', '8', '9', 'C',
        '*', '0', '#', 'D',
        ];
        let freq_map = CHARS
        .into_iter()
        .zip(
            FV.into_iter()
            .flat_map(|fv| FH.into_iter().map(move |fh| (fv, fh))),
        )
        .collect();
        PhoneKeys {
            freq_map
        }
    }
    
    fn add_key(&self, k: &char) -> (Pitch, Pitch) {
        let (f1, f2) = self.freq_map.get(k).unwrap();
        let duration = 150_000_000;
        let p1 = Pitch::new(*f1, duration, 0.9);
        let p2 = Pitch::new(*f2, duration, 0.9);
        (p1,p2)
    }
    fn add_key_d(&self, k: &char, duration: u64) -> (Pitch, Pitch) {
        let (f1, f2) = self.freq_map.get(k).unwrap();
        //let duration = 150_000_000;
        let p1 = Pitch::new(*f1, duration, 0.9);
        let p2 = Pitch::new(*f2, duration, 0.9);
        (p1,p2)
    }
}

fn tastierino(number: &'static str) {
    let phone = PhoneKeys::new();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let (controller, mix) = rodio::dynamic_mixer::mixer(2, 48_000);
    
    let (vv,vh) : (Vec<_>, Vec<_>) = number.chars().map(|c| {
        let (p1,p2) = &phone.add_key_d(&c, 120_000_000);
        (p1.to_source(), p2.to_source())
    }).unzip();
    let s = source::from_iter(vv.into_iter());
    let sh = source::from_iter(vh.into_iter());
    controller.add(s);
    controller.add(sh);
    sink.append(mix);
    sink.sleep_until_end();
}

fn play_pattern(pattern: &Vec<Pitch>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    for p in pattern.iter() {
        let source = p.to_source();
        sink.append(source);
    }
    //sink.append(sources[0]);
    sink.sleep_until_end();
}


#[macro_export]
macro_rules! pd {
    ($v:ident, $f:expr, $d:expr, $a:expr) => {
        $v.push(Pitch::new($f, $d, $a))
    };
}

fn main() {
    let args: Vec<_> = args().collect();
    println!("{:?}",args);
    let number = if args.len() > 1 {
        args[1].clone()
    } else {
        "00".to_string()
    };
    let number = "0733050911113A";
    tastierino(number);

    let mut v: Vec<Pitch> = Vec::new();

    pd!(v, 880.0, 120_000_000, 0.8);
    pd!(v, 880.0, 250_000_000, 0.0);
    pd!(v, 320.0, 50_000_000, 0.0);
    pd!(v, 320.0, 120_000_000, 0.8);
    pd!(v, 320.0, 50_000_000, 0.0);
    pd!(v, 880.0, 120_000_000, 0.8);
    pd!(v, 320.0, 50_000_000, 0.0);
    pd!(v, 880.0, 120_000_000, 0.8);
    pd!(v, 320.0, 50_000_000, 0.0);
    pd!(v, 880.0, 120_000_000, 0.8);
    //pd!(v, 320.0, 50_000_000, 0.0);
    //pd!(v, 880.0, 120_000_000, 0.8);
    pd!(v, 880.0, 250_000_000, 0.0);
    //play_pattern(&v);

    //play_flat();

}