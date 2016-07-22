//! Where stuff is drawn.
//!
//!

#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

// use std::fs::File;
// use std::io::BufReader;
use std::path::Path;
use std::ops::Range;
use glium::{self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface};
use glium::vertex::EmptyInstanceAttributes;
use glium::backend::glutin_backend::GlutinFacade;
use win;
use win::entity::Entity;
// use win::vertex::Vertex;
use util;
use util::Vertex;
use win::models::{self, ModelKind, Models};
use sim::{Snapshot, Object};
// use tobj;
// use obj::{self, Obj, Vertex};


const MAX_ENTITIES: usize = 1024;
static MODEL_FILES: [&'static str; 1] = [
    "/home/nick/models/cube_tri.obj",
];


// Vertex Shader:
static VERTEX_SHADER_SRC: &'static str = r#"
    #version 330
    in vec3 position;
    // in vec4 color;
    in vec3 normal; // 5
    in vec3 translation;
    in float scale;
    out vec4 v_color;
    out vec3 v_position;
    out vec3 v_normal; // <-- line 10

    // uniform mat4 model;
    uniform mat4 view;
    uniform mat4 persp;

    void main() {
        // Model transformation matrix:
        mat4 model = mat4(
            scale, 0.0, 0.0, 0.0,
            0.0, scale, 0.0, 0.0,
            0.0, 0.0, scale, 0.0,
            translation.x, translation.y, translation.z, 1.0
        );

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

#[derive(Copy, Clone, Debug)]
pub struct EntityVertex {
    pub translation: [f32; 3],
    pub orientation: [f32; 4],
    pub scale: f32,
}
implement_vertex!(EntityVertex, translation, orientation, scale);

impl Default for EntityVertex {
    fn default() -> EntityVertex {
        EntityVertex {
            translation: [0.0, 0.0, 0.0],
            orientation: [0.0, 0.0, 0.0, 0.0],
            scale: 0.0,
        }
    }
}


pub struct Entities<'d> {
    // entities: Vec<Entity>,
    entity_groups: Vec<Vec<EntityVertex>>,
    entity_buf: VertexBuffer<EntityVertex>,
    entity_group_ranges: Vec<Range<usize>>,
    models: Models,
    program: Program,
    params: DrawParameters<'d>,
    // vbo: Option<VertexBuffer<Vertex>>,
    // ibo: Option<IndexBuffer<u16>>,
    // models_vbo_ranges: Vec<Range<usize>>,
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


        // let raw_states_vec: Vec<u8> = iter::repeat(0u8).cycle().take(grid_count).collect();
        // let state_vertices: Vec<StateVertex> = iter::repeat(StateVertex { state: 0.0 })
        //     .cycle().take(grid_count).collect();
        // let vertex_buf = VertexBuffer::dynamic(display, &state_vertices).unwrap();
        // let raw_states_buf: Buffer<[u8]> = Buffer::empty_unsized(display, BufferType::ArrayBuffer, grid_count,
        //     BufferMode::Persistent).unwrap();
        // let vec_ref = unsafe { &*(&raw_states_vec as *const Vec<u8>
        //     as *const _ as *const Vec<StateVertex>) };
        let models = Models::new(display);

        let entity_groups = vec![Vec::with_capacity(128); models.count()];
        let entity_group_ranges = Vec::with_capacity(models.count());

        // // DEBUG:
        // println!("entity_groups.len() = {}", entity_groups.len());

        // [NOTE]: `persistent` gives performance improvement:
        // let raw_states_buf = VertexBuffer::dynamic(display, vec_ref).unwrap();
        let entity_buf = VertexBuffer::empty_persistent(display, MAX_ENTITIES).unwrap();


        Entities {
            entity_groups: entity_groups,
            entity_buf: entity_buf,
            models: models,
            entity_group_ranges: entity_group_ranges,
            program: program,
            params: params,
            // vbo: None,
            // ibo: None,
            display: display,
            // surface_dims: display.get_framebuffer_dimensions(),
            light_pos: [-1.0, 0.4, -0.9f32],
            global_color: [0.2, 0.2, 0.6f32],
            // cam_pos_raw: [0.0, 0.0, -1.0],
        }
    }

    pub fn init(mut self) -> Entities<'d> {
        // // let (vertices, indices) = models::hexagon_panel(0.3, 0.0, 1.0, win::C_ORANGE);
        // // self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
        // // self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
        // //     &indices).unwrap());

        // let vertices = util::load_wavefront(&Path::new("/home/nick/models/cube_tri.obj"));

        // self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());

        // // LOADING AN EMPTY INDEX BUFFER (temporary... to satisfy ::draw() init check):
        // self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
        //     &vec![]).unwrap());

        // // self.ibo = Some(glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList));

        self
    }

    pub fn draw<S: Surface>(&mut self, surface: &mut S, sim: &Snapshot, cam_pos: &[f32; 3]) {
        self.update_entities(sim);

        // Surface dims:
        let surface_dims = surface.get_dimensions();

        // Perspective transformation matrix:
        let persp = util::persp_matrix(surface_dims.0, surface_dims.1, 3.0);

        // View transformation matrix: { position(x,y,z), direction(x,y,z), up_dim(x,y,z)}
        let view = util::view_matrix(cam_pos, &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

        // // Update mouse focus:
        // self.update_mouse_focus();

        // // Draw entities:
        // surface.draw((self.models.vbo(), EmptyInstanceAttributes { len: 1 }),
        //     &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        //     &self.program, &uniforms, &self.params).unwrap();

        // Draw entities for each model type:
        for (model_id, range) in self.entity_group_ranges.iter().enumerate() {
        // for model_id in 0..self.models.count() {
            // // Set up model size/scale:
            // let scl = 1.0;

            // // Set up model position:
            // let x_shift = -6.5;
            // let y_shift = -6.5;
            // let z_shift = 14.0;

            // // Model transformation matrix:
            // let model = [
            //     [scl, 0.0, 0.0, 0.0],
            //     [0.0, scl, 0.0, 0.0],
            //     [0.0, 0.0, scl, 0.0],
            //     [x_shift, y_shift, z_shift, 1.0f32],
            // ];

            // Uniforms:
            let uniforms = uniform! {
                // model: model,
                view: view,
                persp: persp,
                u_light_pos: self.light_pos,
                u_global_color: self.global_color,
            };

            let ent_buf_slice = self.entity_buf.slice(range.clone()).unwrap();

            // // DEBUG:
            // println!("Drawing entity group with range: {:?}", range.clone());

            surface.draw((self.models.verts(model_id),
                    ent_buf_slice.per_instance().unwrap()
                    // EmptyInstanceAttributes { len: 1 }
                ),
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program, &uniforms, &self.params).unwrap();
        }


        // // Draw element text:
        // for element in self.elements.iter() {
        //     element.draw_text(&self.text_system, target, &self.font_texture);

        //     let text_display = TextDisplay::new(&self.text_system, &self.font_texture,
        //         element.get_text());

        //     glium_text::draw(&text_display, &self.text_system, target,
        //         element.text_matrix(), element.text().get_color());
        // }
    }

    pub fn models(&self) -> &Models {
        &self.models
    }

    // [TODO]: This system needs to be redesigned. There are too many steps
    // between sim and drawing (see README notes).
    //
    // Also: Only `Object::Entity` is being drawn.
    pub fn update_entities(&mut self, sim: &Snapshot) {
        // Clear entity groups:
        for group in self.entity_groups.iter_mut() {
            group.clear();
        }

        // Convert `Node` to `EntityVertex` and place into appropriate group bin.
        for o_node in sim.nodes() {
            match *o_node {
                Some(ref node) => {
                    let model_id = match node.payload {
                        // Only `Entity` being drawn:
                        Object::Entity(e) => e.unwrap(),
                        // Skipping all other object types:
                        _ => continue,
                    };

                    self.entity_groups[model_id].push(EntityVertex {
                        translation: node.position,
                        orientation: node.orientation,
                        scale: node.size,
                    });
                },
                None => (),
            }
        }

        self.update_vertices();
    }

    /// Populates `entity_buf` (entities vertex buffer) from `entity_groups`.
    fn update_vertices(&mut self) {
        self.entity_group_ranges.clear();
        let mut buf_len = 0usize;
        let mut buf_map = self.entity_buf.map();

        for (group_id, group) in self.entity_groups.iter().enumerate() {
            debug_assert!(self.entity_group_ranges.len() == group_id);
            self.entity_group_ranges.push(buf_len..group.len());

            // // DEBUG:
            // println!("Adding entity group with range: {:?}", buf_len..group.len());

            for ent_vert in group.iter() {
                buf_map[buf_len] = ent_vert.clone();
                buf_len += 1;
            }

            debug_assert!(buf_len == self.entity_group_ranges.last().unwrap().start + group.len());
        }
    }
}
