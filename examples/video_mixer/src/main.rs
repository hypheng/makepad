/*
TeamTalk is a LAN (only) p2p audiochat supporting as many clients as you have bandwidth.
For 6 clients it should pull about 25 megabits. You can use it to have a super low latency
helicopter-headset experience, silent disco, and so on.
*/

pub use makepad_audio_graph;
pub use makepad_audio_graph::makepad_widgets;
pub use makepad_audio_graph::makepad_platform;

use {
    crate::{
        makepad_widgets::*,
        //makepad_platform::video_capture::*,
        makepad_draw::*,
    },
};

// We dont have a UI yet

live_design!{
    import makepad_widgets::frame::*;
    import makepad_draw::shader::std::*;
    registry Widget::*;
    App = {{App}} {
        ui: {
            walk: {width: Fill, height: Fill},
            draw_bg: {
                shape: Rect
                fn pixel(self) -> vec4 {
                    return Pal::premul(#3)
                }
            }
        }
    }
}
main_app!(App);

#[derive(Live, LiveHook)]
pub struct App {
    window: DesktopWindow,
    ui: FrameRef,
}

impl App {
    pub fn live_design(cx: &mut Cx) {
        makepad_audio_graph::live_design(cx);
    }
    
    pub fn start_inputs(&mut self, cx: &mut Cx) {
        cx.audio_input(0, move | _device, _time, input_buffer | {
            input_buffer
        });
        
        cx.audio_output(0, move | _device, _time, _output_buffer | {
        });
        
        cx.video_capture(0, move |img|{
            println!("GOT {}", img.data.len()); 
        })
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // no UI as of yet
        match event {
            Event::Draw(event) => {
                return self.draw(&mut Cx2d::new(cx, event));
            }
            Event::Construct => {
                self.start_inputs(cx);
            }
            Event::MidiPorts(ports) => {
                cx.use_midi_inputs(&ports.all_inputs());
            }
            Event::AudioDevices(devices) => { 
                cx.use_audio_inputs(&devices.default_input());
                cx.use_audio_outputs(&devices.default_output());
            }
            Event::VideoCaptureDevices(devices)=>{
                //println!("{:?}", devices);
                cx.use_video_capture(&devices.find_highest(0));
            }
            _ => ()
        }
         
        self.ui.handle_event(cx, event);
        self.window.handle_event(cx, event);
    }
    
    pub fn draw(&mut self, cx: &mut Cx2d) { 
        if self.window.begin(cx).not_redrawing() {
            return;
        }
        
        while self.ui.draw(cx).is_not_done() {};
        
        //self.ui.redraw(cx);
        self.window.end(cx);
    }
}