//! Where stuff is drawn.
//!
//!

#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

// use std::fs::File;
// use std::io::BufReader;
use std::path::Path;

use glium::{self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface};
use glium::vertex::{EmptyInstanceAttributes as Eia};
use glium::backend::glutin_backend::GlutinFacade;
use win;
use win::entity::Entity;
// use win::vertex::Vertex;
use util::Vertex;
use win::models;
use util;
use sim::Snapshot;
// use tobj;
// use obj::{self, Obj, Vertex};


// Vertex Shader:
static VERTEX_SHADER_SRC: &'static str = r#"
    #version 330
    in vec3 position;
    // in vec4 color;
    in vec3 normal;
    out vec4 v_color;
    out vec3 v_position;
    out vec3 v_normal;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 persp;

    void main() {
        mat4 model_view = view * model;

        gl_Position = persp * model_view * vec4(position, 1.0);
        v_normal = transpose(inverse(mat3(model_view))) * normal;
        // v_color = color;
        v_position = gl_Position.xyz / gl_Position.w;

        // gl_Position = vec4(position, 1.0);
        // v_color = color;
    };
"#;

// Fragment Shader:
static FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 330
    // in vec4 v_color;
    in vec3 v_normal;
    in vec3 v_position;
    out vec4 color;

    uniform vec3 u_light_pos;
    uniform vec3 u_global_color;

    // const float ambient_strength = 0.1;
    const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
    const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
    const vec3 specular_color = vec3(0.3, 0.3, 0.3);
    const float specular_coeff = 16.0;

    void main() {
        float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0),
            specular_coeff);

        float state_norm = 255.0 / 255.0;
        // vec3 tile_color = vec3(state_norm, u_global_color.g, u_global_color.b);
        vec3 tile_color = vec3(u_global_color.r, u_global_color.g, u_global_color.b);

        color = vec4((ambient_color * tile_color) + diffuse_ampl
            * diffuse_color + specular * specular_color, 1.0);
    };
"#;


pub struct Entities<'d> {
    entities: Vec<Entity>,
    program: Program,
    params: DrawParameters<'d>,
    vbo: Option<VertexBuffer<Vertex>>,
    ibo: Option<IndexBuffer<u16>>,
    display: &'d GlutinFacade,
    // surface_dims: (u32, u32),
    light_pos: [f32; 3],
    global_color: [f32; 3],
    // cam_pos_raw: [f32; 3],
}

impl<'d> Entities<'d> {
    pub fn new(display: &'d GlutinFacade) -> Entities {
        let scale = 1.0f32;

        let program = Program::from_source(display, VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC, None).unwrap();

        // Draw parameters:
        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled, // <-- default
            .. Default::default()
        };

        Entities {
            entities: Vec::with_capacity(128),
            program: program,
            params: params,
            vbo: None,
            ibo: None,
            display: display,
            // surface_dims: display.get_framebuffer_dimensions(),
            light_pos: [-1.0, 0.4, -0.9f32],
            global_color: [0.2, 0.2, 0.6f32],
            // cam_pos_raw: [0.0, 0.0, -1.0],
        }
    }

    pub fn init(mut self) -> Entities<'d> {
        // let (vertices, indices) = models::hexagon_panel(0.3, 0.0, 1.0, win::C_ORANGE);
        // self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
        // self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
        //     &indices).unwrap());

        let vertices = util::load_wavefront(&Path::new("/home/nick/models/cube_tri.obj"));

        self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());

        // LOADING AN EMPTY INDEX BUFFER (temporary... to satisfy ::draw() init check):
        self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
            &vec![]).unwrap());

        // self.ibo = Some(glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList));

        self
    }

    pub fn draw<S: Surface>(&mut self, surface: &mut S, sim: &Snapshot, cam_pos: &[f32; 3]) {
        if self.vbo.is_none() || self.ibo.is_none() {
            panic!("Entities::draw(): Buffers not initialized.")
        }

        // Surface dims:
        let surface_dims = surface.get_dimensions();

        // Perspective transformation matrix:
        let persp = util::persp_matrix(surface_dims.0, surface_dims.1, 3.0);

        // View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
        let view = util::view_matrix(cam_pos, &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

        let scl = 1.0;

        // Set up model position:
        let x_shift = -1.5;
        let y_shift = -1.5;
        let z_shift = 8.0;

        // Model transformation matrix:
        let model = [
            [scl, 0.0, 0.0, 0.0],
            [0.0, scl, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_shift, y_shift, z_shift, 1.0f32],
        ];

        // Uniforms:
        let uniforms = uniform! {
            model: model,
            view: view,
            persp: persp,
            u_light_pos: self.light_pos,
            u_global_color: self.global_color,
            // grid_v_size: grid_dims.0,
            // grid_u_size: grid_dims.1,
        };

        // // Update mouse focus:
        // self.update_mouse_focus();

        // Draw entities:
        // surface.draw((self.vbo.as_ref().unwrap(), Eia { len: 1 }), self.ibo.as_ref().unwrap(),
        //     &self.program, &uniforms, &self.params).unwrap();
        surface.draw((self.vbo.as_ref().unwrap(), Eia { len: 1 }),
            // self.ibo.as_ref().unwrap(),
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &self.program, &uniforms, &self.params).unwrap();


        // // Draw element text:
        // for element in self.elements.iter() {
        //     element.draw_text(&self.text_system, target, &self.font_texture);

        //     let text_display = TextDisplay::new(&self.text_system, &self.font_texture,
        //         element.get_text());

        //     glium_text::draw(&text_display, &self.text_system, target,
        //         element.text_matrix(), element.text().get_color());
        // }
    }

    fn update_positions(&mut self, t: u64) {
        for entity in self.entities.iter() {
            // entity.update_position()
        }
    }
}
