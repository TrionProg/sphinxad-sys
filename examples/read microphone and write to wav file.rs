//Add to Cargo.toml
[dependencies]
sphinxad-sys = "*"
hound = "*"

//====================

extern crate sphinxad_sys;
extern crate hound;

use std::thread;
use std::ptr;

use sphinxad_sys::*;

const SPHINX_FREQ:u32=16000; //sphinx want 16hHz

fn main(){
    //specify wav file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SPHINX_FREQ,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();

    //open the Microphone(device)
    let device=unsafe{
        let device=ad_open_sps(SPHINX_FREQ);

        if device.is_null() {
            panic!("no device");
        }

        if ad_start_rec(device)!=0 {
            panic!("can not start recording");
        }

        device
    };

    println!("speak");

    //read ~3secs and write to file
    let mut buffer = [0i16; 2048];

    for i in 0..30{ //note, this loop wont works 100*30 ms
        let raw=&mut buffer[0] as *mut i16;
        let n=unsafe{ ad_read(device, raw, buffer.len() as u32) };

        if n<0 {
            println!("can not read audio");
            break;
        }

        for i in 0..n as usize {
            writer.write_sample(buffer[i]).unwrap();
        }

        thread::sleep_ms(100);
    }

    //do not forget to close device
    unsafe{
        ad_stop_rec(device);
        ad_close(device);
    }
}
