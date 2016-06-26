//! Window into the world.
//!
//!

#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

mod creature;
mod entity;
mod entities;
mod vertex;
mod models;

use glium::{self, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{self, WindowBuilder, Event};
use self::entities::Entities;
use sim;

pub const TOP_LEFT: [f32; 3] = [-1.0, 1.0, 0.0];
pub const TOP_RIGHT: [f32; 3] = [1.0, 1.0, 0.0];
pub const BOTTOM_LEFT: [f32; 3] = [-1.0, -1.0, 0.0];
pub const BOTTOM_RIGHT: [f32; 3] = [1.0, -1.0, 0.0];

pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0];
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];

pub const DEFAULT_CAM_DST: f32 = 1.5;


pub struct Window {
    surface_dims: (u32, u32),
    close_pending: bool,
    cam_pos_raw: [f32; 3],
    cam_pos_norm: [f32; 3],
    mouse_pos: (i32, i32),
    has_mouse_focus: bool,
    dragging: Option<(i32, i32)>,
    // entities: Entities<'d>,
}

impl<'d> Window {
    pub fn open() {
        println!("Opening a window into the world...");

        let display: GlutinFacade = WindowBuilder::new()
            .with_depth_buffer(24)
            .with_dimensions(1400, 800)
            .with_title("Vibi".to_string())
            // .with_multisampling(8)
            // Disabled for development ->> .with_gl_robustness(glium::glutin::Robustness::NoError)
            .with_vsync()
            // .with_transparency(true)
            // .with_fullscreen(glium::glutin::get_primary_monitor())
            .build_glium().unwrap();

        let mut sim = sim::Snapshot::new();
        sim.new_entity(sim::Object::Cube);

        let mut entities = Entities::new(&display).init();

        // Main window data struct:
        let mut window = Window {
            close_pending: false,
            surface_dims: display.get_framebuffer_dimensions(),
            cam_pos_raw: [0.0, 0.0, -1.0],
            cam_pos_norm: [0.0, 0.0, DEFAULT_CAM_DST],
            mouse_pos: (0, 0),
            has_mouse_focus: true,
            dragging: None,
        };


        //////////////////////////////////////////////////////////////////////////
        ///////////////////// Primary Event & Rendering Loop /////////////////////
        //////////////////////////////////////////////////////////////////////////
        loop {
            // Create draw target and clear color and depth:
            let mut surface = display.draw();
            surface.clear_color_and_depth((0.030, 0.050, 0.080, 1.0), 1.0);
            window.surface_dims = surface.get_dimensions();

            // // Get read for new input:
            // ui.set_input_stale();

            // Check input events:
            for ev in display.poll_events() {
                // window.handle_event_remainder(ui.handle_event(ev));
                window.handle_event(ev);
            }

            // // Draw UI:
            // ui.draw(&mut surface);

            // Draw entities:
            entities.draw(&mut surface, &sim, &window.cam_pos_raw);

            // Swap buffers:
            surface.finish().unwrap();

            // Clean up and exit if necessary:
            if window.close_pending {
                break;
            }
        }

        // Hide window when exiting.
        // [FIXME] TODO: Draw "Closing..." or something like that to the display instead.
        display.get_window().unwrap().hide();
    }

    fn handle_event(&mut self, ev: Event) {
        match ev {
            // Event::KeyboardInput(state, _, v_code) => ()
            //     println!("Key: {:?} has been {:?}", ui::map_vkc(v_code), state),
            // Event::MouseMoved(p_x, p_y) => self.handle_mouse_moved((p_x, p_y)),
            // Event::MouseWheel(delta, _) => self.handle_mouse_wheel(delta),
            // Event::MouseInput(state, btn) => self.handle_mouse_input(state, btn),
            // Event::Touch(touch) => println!("Touch recieved: {:?}", touch),
            // Event::Resized => self.surface_dims = surface.get_dimensions(),
            Event::Closed => self.close_pending = true,
            _ => (),
        }
    }

    // fn handle_event_remainder(&mut self, rdr: WindowCtl) {
    //     match rdr {
    //         WindowCtl::None => (),
    //         WindowCtl::Event(event) => { match event {
    //             // Event::KeyboardInput(state, _, v_code) => ()
    //             //     println!("Key: {:?} has been {:?}", ui::map_vkc(v_code), state),
    //             Event::MouseMoved(p_x, p_y) => self.handle_mouse_moved((p_x, p_y)),
    //             Event::MouseWheel(delta, _) => self.handle_mouse_wheel(delta),
    //             Event::MouseInput(state, btn) => self.handle_mouse_input(state, btn),
    //             Event::Touch(touch) => println!("Touch recieved: {:?}", touch),
    //             Event::Closed => self.close_pending = true,
    //             _ => (),
    //         } }
    //         WindowCtl::CyCtl(ctl) => self.control_tx.send(ctl).unwrap(),
    //         WindowCtl::SetCyIters(i) => self.iters_pending = i,
    //         WindowCtl::CyIterate => self.control_tx.send(CyCtl::Iterate(self.iters_pending)).unwrap(),
    //         WindowCtl::HexGrid(ctl) => {
    //             match ctl {
    //                 HexGridCtl::SlcRangeDefault => self.hex_grid.buffer.use_default_slc_range(),
    //                 HexGridCtl::SlcRangeFull => self.hex_grid.buffer.use_full_slc_range(),
    //             }
    //             self.hex_grid.update_cam_pos();
    //         },
    //         // _ => (),
    //     }
    // }

    // /// Moves the camera position in our out (horizontal scrolling ignored).
    // #[allow(dead_code)]
    // fn handle_mouse_wheel(&mut self, scroll_delta: MouseScrollDelta) {
    //     let (hrz, vrt) = match scroll_delta {
    //         MouseScrollDelta::LineDelta(h, v) => (h * 10.0, v * 10.0),
    //         MouseScrollDelta::PixelDelta(x, y) => (x, y),
    //     };
    //     let _ = hrz;

    //     self.hex_grid.zoom_camera(vrt);
    // }

    // fn handle_mouse_moved(&mut self, pos: (i32, i32)) {
    //     self.mouse_pos = pos;

    //     if let Some(ref mut start_pos) = self.dragging {
    //         let delta = (pos.0 - start_pos.0, pos.1 - start_pos.1);
    //         self.hex_grid.move_camera(delta);
    //         *start_pos = pos;
    //     }
    // }

    // #[allow(dead_code, unused_variables)]
    // fn handle_mouse_input(&mut self, button_state: ElementState, button: MouseButton) {
    //     match button {
    //         MouseButton::Left => {
    //             match button_state {
    //                 ElementState::Pressed => self.dragging = Some(self.mouse_pos),
    //                 ElementState::Released => self.dragging = None,
    //             }
    //         },
    //         _ => (),
    //     }
    // }


        // [TODO]: Simplify this mess and try to get it more accurate.
    pub fn update_cam_pos(&mut self) {
        let aspect_ratio = self.surface_dims.1 as f32 / self.surface_dims.0 as f32;
        // let slc_count = self.buffer.cur_slc_range().len();

        let x_ofs = 75.0;
        let cam_x_pos = self.cam_pos_norm[0].mul_add(-1000.0, x_ofs);

        let y_ofs = -0.0;
        let cam_y_pos = self.cam_pos_norm[1].mul_add(1000.0, y_ofs);

        let z_ofs = -0.01;
        let cam_z_pos = self.cam_pos_norm[2].mul_add(-53.0, z_ofs);

        self.cam_pos_raw = [cam_x_pos, cam_y_pos, cam_z_pos];
        // println!("CAMERA POSITION: norm: {:?}, raw: {:?}", self.cam_pos_norm, self.cam_pos_raw);
    }

    pub fn move_camera(&mut self, delta: (i32, i32)) {
        let delta_x = delta.0 as f32 / self.surface_dims.0 as f32;
        let new_cam_x = self.cam_pos_norm[0] + delta_x;
        let new_x_valid = (new_cam_x >= -1.0 && new_cam_x <= 1.0) as i32 as f32;
        self.cam_pos_norm[0] = new_x_valid.mul_add(delta_x, self.cam_pos_norm[0]);

        let delta_y = delta.1 as f32 / self.surface_dims.1 as f32;
        let new_cam_y = self.cam_pos_norm[1] + delta_y;
        let new_y_valid = (new_cam_y >= -1.0 && new_cam_y <= 1.0) as i32 as f32;
        self.cam_pos_norm[1] = new_y_valid.mul_add(delta_y, self.cam_pos_norm[1]);

        self.update_cam_pos();
    }

    pub fn zoom_camera(&mut self, delta: f32) {
        let delta_z = delta * -0.001;
        let new_cam_z = self.cam_pos_norm[2] + delta_z;
        let new_z_valid = (new_cam_z >= 0.00 && new_cam_z <= 10.00) as i32 as f32;
        self.cam_pos_norm[2] = new_z_valid.mul_add(delta_z, self.cam_pos_norm[2]);

        self.update_cam_pos();
    }
}

// impl<'d> SetMouseFocus for Window<'d> {
//     fn set_mouse_focus(&mut self, focus: bool) {
//         self.has_mouse_focus = focus;
//         // println!("WINDOW::SET_MOUSE_FOCUS(): Setting focus to: {}, dragging: {:?}", focus, self.dragging);
//     }
// }