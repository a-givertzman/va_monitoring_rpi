#![allow(non_snake_case)]

use std::{
    sync::{
        Arc, 
        Mutex
    }, 
    time,
    thread,
    error::Error,
};
// use egui::mutex::Mutex;

pub const PI: f32 = std::f32::consts::PI;
pub const PI2: f32 = PI * 2.0;

type BuilderCallback = fn(t: f32, f: f32) -> f32;

///
/// 
pub struct InputSignal {
    handle: Option<thread::JoinHandle<()>>,
    cancel: bool,
    pub f: f32,
    pub period: f32,
    builder: BuilderCallback,
    len: usize,
    step: f32,
    pub t: f32,
    pub phi: f32,
    pub amplitude: f32,
    pub xyPoints: Vec<[f64; 2]>,
    // pub complex0: Vec<Complex<f32>>,
    // pub complex0Current: Vec<[f64; 2]>,
    // pub complex: Vec<Complex<f32>>,
    // pub complexCurrent: Vec<[f64; 2]>,
    // pub xyPoints: Vec<[f64; 2]>,
}
impl InputSignal {
    ///
    /// Creates new instance
    pub fn new(f: f32, builder: BuilderCallback, len: usize, step: Option<f32>) -> Self {
        let period = 1.0 / f;
        let delta = period / (len as f32);
        println!("[InputSignal] f: {:?} Hz", f);
        println!("[InputSignal] T: {:?} sec", period);
        println!("[InputSignal] N: {:?} poins", len);
        println!("[InputSignal] delta: {:?} sec", delta);
        Self { 
            handle: None,
            cancel: false,
            f: f,
            period: 1.0 / f,
            builder,
            len: len,
            step: match step {
                Some(value) => value,
                None => delta,
            },
            t: 0.0,
            phi: 0.0,
            amplitude: 0.0,
            // origin: vec![0.0],
            // complex0: vec![Complex{re: 0.0, im: 0.0}],
            // complex0Current: vec![[0.0, 0.0], [0.0, 0.0]],
            // complex: vec![Complex{re: 0.0, im: 0.0}],
            // complexCurrent: vec![[0.0, 0.0], [0.0, 0.0]],
            xyPoints: vec![[0.0, 0.0]],
        }
    }
    ///
    /// Starts in the thread
    pub fn run(this: Arc<Mutex<Self>>) -> Result<(), Box<dyn Error>> {
        let cancel = this.lock().unwrap().cancel;
        let me = this.clone();
        let handle = Some(
            thread::Builder::new().name("InputSignal tread".to_string()).spawn(move || {
                println!("[InputSignal] started in {:?}", thread::current().name().unwrap());
                while !cancel {
                    // println!("tread: {:?} cycle started", thread::current().name().unwrap());
                    this.lock().unwrap().next();
                    thread::sleep(time::Duration::from_nanos(10000));
                }
            })?
        );
        me.lock().unwrap().handle = handle;
        Ok(())
    }
    ///
    /// Stops thread
    pub fn cancel(&mut self) {
        self.cancel = true;
    }
    /// 
    /// Calculates all new values with new time
    fn next(&mut self) {
        // let th = thread::current();
        // let thName = th.name().unwrap();
        // println!("tread: {:?} next started", thName);
        self.t = self.t + self.step;

        let PI2f = PI2 * self.f;
        self.phi += PI2f * self.step;
        if self.phi > PI2f * self.period {
            self.phi = 0.0;
        }
        
        // self.inputFilter.add((self.builder)(t, self.f));
        // let input = self.inputFilter.value();
        self.amplitude = (self.builder)(self.t, self.f);
        self.xyPoints.push([self.t as f64, self.amplitude as f64]);
        if self.xyPoints.len() > self.len {
            // println!("[InputSignal] length excidded {:?}", thread::current().name().unwrap());
            self.xyPoints.remove(0);
        }
        // println!("tread: {:?} amplitude {}", thName, self.amplitude);

        // let PI2ft = PI2f * t;
        // let re0 = (PI2ft).cos();
        // let im0 = (PI2ft).sin();

        // let re = input * (PI2ft).cos();
        // let im = input * (PI2ft).sin();
        // println!("complex: {:?}", complex);
    }
    ///
    /// current value [time, amplitude]
    pub fn read(&self) -> [f32; 3] {
        [self.phi, self.t, self.amplitude]
    }
}
