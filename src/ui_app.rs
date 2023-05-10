#![allow(non_snake_case)]

use log::{
    // info,
    // trace,
    debug,
    // warn,
};
use std::{
    sync::{
        Arc, 
        Mutex
    }, 
    time::Duration
};
use egui::{
    plot::{
        Plot, 
        Points, 
        // PlotPoints, 
        Line
    }, 
    Color32, 
    // mutex::Mutex,
};
use crate::{
    analyze_fft::{
        AnalizeFft
    }, 
    input_signal::{
        InputSignal,
        PI,
    }, udp_server::UdpServer
};



pub struct UiApp {
    pub inputSignal: Arc<Mutex<InputSignal>>,
    pub analyzeFft: Arc<Mutex<AnalizeFft>>,
    pub udpSrv: Arc<Mutex<UdpServer>>,
    renderDelay: Duration,
    text: String,
}

impl UiApp {
    pub fn new(
        inputSignal: Arc<Mutex<InputSignal>>, 
        analyzeFft: Arc<Mutex<AnalizeFft>>,
        udpSrv: Arc<Mutex<UdpServer>>,
        renderDelay: Duration,
    ) -> Self {
        Self {
            inputSignal: inputSignal, 
            analyzeFft: analyzeFft,
            udpSrv: udpSrv,
            renderDelay: renderDelay,
            text: String::from(""),
        }
    }
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let phi = self.inputSignal.lock().unwrap().phi;
        let f = self.inputSignal.lock().unwrap().f;
        let period = self.inputSignal.lock().unwrap().period;
        
        // egui::Window::new("complex 0").show(ctx, |ui| {
        //     let mut analyzeFft = self.analyzeFft.lock().unwrap();
        //     // ui.label(format!("complex 0: '{}'", 0));
        //     ui.label(format!(" f: {:?} Hz   T: {:?} sec", f, period));
        //     ui.label(format!(" pfi: {:?}", phi * 180.0 / PI));
        //     ui.label(format!(" complex 0 len: {:?}", self.inputSignal.lock().unwrap().complex0.len()));
        //     ui.end_row();
        //     if ui.button("Stop").clicked() {
        //         analyzeFft.cancel();
        //     }
        //     Plot::new("complex 0")
        //     .show(ui, |plotUi| {
        //         let points: Vec<[f64; 2]> = self.inputSignal.lock().unwrap().complex0.iter().map(|complex| {
        //             [complex.re, complex.im]
        //         }).collect();
        //         let i = self.inputSignal.lock().unwrap().i;
        //         plotUi.points(
        //             Points::new(
        //                 points.clone(),
        //             ).color(Color32::BLUE),
        //         );
        //         if points.len() > 2 {
        //             plotUi.line(
        //                 Line::new(
        //                     vec![[0.0; 2], points[i-1]],
        //                 )
        //                 .color(Color32::YELLOW),
        //             );
        //         }
        //     });
        // });
        // egui::Window::new("input complex").show(ctx, |ui| {
        //     // let analyzeFft = self.analyzeFft.lock().unwrap();
        //     // ui.label(format!("complex 0: '{}'", 0));
        //     ui.label(format!(" f: {:?} Hz   T: {:?} sec", f, period));
        //     ui.label(format!(" pfi: {:?}", phi * 180.0 / PI));
        //     ui.end_row();
        //     let textEdit = ui.text_edit_singleline(&mut self.text);
        //     if textEdit.changed() {
        //         debug!("text edited: {:?}", self.text);
        //     };
        //     if textEdit.lost_focus() {
        //         debug!("text editing finished: {:?}", self.text);
        //     };
        //     if ui.button("just button").clicked() {
        //     }
        //     Plot::new("complex")
        //     .show(ui, |plotUi| {
        //         let points: Vec<[f64; 2]> = self.inputSignal.lock().unwrap().complex.buffer().iter().map(|complex| {
        //             [complex.re, complex.im]
        //         }).collect();
        //         plotUi.points(
        //             Points::new(
        //                 points.clone(),
        //             ).color(Color32::BLUE),
        //         );
        //         let i = self.inputSignal.lock().unwrap().i;
        //         plotUi.line(
        //             Line::new(
        //                 vec![[0.0; 2], points[i]],
        //             )
        //             .color(Color32::YELLOW),
        //         );
        //     });
        // });

        egui::Window::new("input signal").show(ctx, |ui| {
            let mut inputSignal = self.inputSignal.lock().unwrap();
            ui.label(format!(" i: {:?}", inputSignal.i));
            ui.label(format!(" t: {:?}", inputSignal.t));
            ui.label(format!(" phi: {:?}", inputSignal.phi));
            // ui.label(format!(" t: {:?}", inputSignal.t));
            ui.label(format!("length: {}", inputSignal.xyPoints.len()));
            // ui.label(format!("xyPoints length: {}", inputSig.xyPoints.len()));
            // ui.end_row();
            if ui.button("Stop").clicked() {
                inputSignal.cancel();
            }
            Plot::new("inputsignal").show(ui, |plotUi| {
                plotUi.points(
                    Points::new(
                        inputSignal.xyPoints.buffer().clone()
                    ),
                )
            });
        });

        egui::Window::new("real input").show(ctx, |ui| {
            // debug!("[UiApp.update] self.udpSrv.lock...");
            match self.udpSrv.lock() {
                Ok(inputSignal) => {
                    // debug!("[UiApp.update] self.udpSrv.lock ready");
                    // ui.label(format!(" i: {:?}", inputSignal.i));
                    ui.label(format!(" t: {:?}", inputSignal.t));
                    // ui.label(format!(" phi: {:?}", inputSignal.phi));
                    ui.label(format!("length: {}", inputSignal.xy.len()));
                    // ui.label(format!("xyPoints length: {}", inputSig.xyPoints.len()));
                    // ui.end_row();
                    if ui.button("Stop").clicked() {
                        // inputSignal.cancel();
                    }
                    Plot::new("real input").show(ui, |plotUi| {
                        plotUi.points(
                            Points::new(
                                inputSignal.xy.buffer().clone()
                            ),
                        )
                    });
                },
                Err(err) => {
                    debug!("[UiApp.update] self.udpSrv.lock error: {:?}", err);
                },
            };
        });

        // egui::Window::new("AnalyzeFft input").show(ctx, |ui| {
        //     let analyzeFft = self.analyzeFft.lock().unwrap();
        //     ui.label(format!(" t: {:?}", analyzeFft.t));
        //     ui.label(format!("t length: {}", analyzeFft.tList.len()));
        //     ui.label(format!("xyPoints length: {}", analyzeFft.xyPoints.len()));
        //     // ui.end_row();
        //     if ui.button("just button").clicked() {
        //     }
        //     Plot::new("input").show(ui, |plotUi| {
        //         plotUi.points(
        //             Points::new(
        //                 analyzeFft.xyPoints.buffer().clone(),
        //             ),
        //         )
        //     });
        // });
        egui::Window::new("fft").show(ctx, |ui| {
            let analyzeFft = self.udpSrv.lock().unwrap();
            // ui.label(format!("new fft: '{}'", 0));
            let points = analyzeFft.fftPoints();
            ui.label(format!("fftComplex length: {}", analyzeFft.fftComplex.len()));
            ui.label(format!("fftPoints length: {}", points.len()));
            if ui.button("just button").clicked() {
            }
            Plot::new("fft").show(ui, |plotUi| {
                plotUi.line(
                    Line::new(
                        points,
                    ).color(Color32::LIGHT_GREEN),
                )
            });
        });
        std::thread::sleep(self.renderDelay);
        ctx.request_repaint();
    }
}
