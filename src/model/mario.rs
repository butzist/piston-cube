use cgmath::prelude::*;
use cgmath::{Matrix3, Matrix4, Vector3};
use foreach::ForEach;
use gfx::traits::*;
use obj::{Mtl, Obj, SimplePolygon};
use piston_window::*;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

#[derive(Clone)]
pub struct Mario {
    groups: Vec<super::ObjectData>,
    start_time: std::time::SystemTime,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    time_offset: f32,
}

impl Mario {
    pub fn new(
        pipeline: Arc<Mutex<crate::pipeline::ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
    ) -> Mario {
        use std::io::BufReader;

        let target = "https://www.models-resource.com/download/685/";
        let model_file = BufReader::new(super::download_cached(target).unwrap());
        let mut zip_archive = ZipArchive::new(model_file).unwrap();

        let mut obj = Obj::<SimplePolygon>::load_buf(&mut BufReader::new(
            zip_archive.by_name("mariohead.obj").unwrap(),
        ))
        .unwrap();

        let textures = load_materials(&mut obj, &mut zip_archive, factory);

        let vertices: Vec<_> = obj
            .position
            .iter()
            .zip(obj.texture.iter().zip(obj.normal.iter()))
            .map(|(position, (texture, normal))| {
                super::Vertex::new(position.clone(), normal.clone(), texture.clone())
            })
            .collect();

        let vbuf = factory.create_vertex_buffer(&vertices);

        let groups: Vec<_> = obj.objects[0]
            .groups
            .iter()
            .map(|g| {
                let indices: Vec<u16> = g
                    .polys
                    .iter()
                    .flat_map(|p| p.iter().map(|t| t.0 as u16))
                    .collect();
                let tex_name = g.material.as_ref().unwrap().map_kd.as_ref().unwrap();
                super::ObjectData::new_from_existing(
                    pipeline.clone(),
                    factory,
                    vbuf.clone(),
                    &indices,
                    textures[tex_name].clone(),
                )
            })
            .collect();
        drop(textures);

        Mario {
            groups,
            start_time: std::time::SystemTime::now(),
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            time_offset: 0.0,
        }
    }

    pub fn update(&mut self) {
        let t = self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let position = t * self.velocity + self.position;

        let model_rotation =
            Matrix3::from_axis_angle([0.0f32, 1.0, 0.0].into(), cgmath::Rad(t + self.time_offset));

        let matrix = Matrix4::from_translation(position)
            * Matrix4::from_scale(0.1)
            * Matrix4::from(model_rotation);
        let matrix_normal = model_rotation.invert().unwrap().transpose();

        for model in &mut self.groups {
            model.matrix = matrix;
            model.matrix_normal = matrix_normal;
        }
    }

    pub fn clone_to<P: Into<Vector3<f32>>, V: Into<Vector3<f32>>>(
        &self,
        position: P,
        velocity: V,
        time_offset: f32,
    ) -> Mario {
        let mut new = self.clone();
        new.position = position.into();
        new.velocity = velocity.into();
        new.time_offset = time_offset;
        new
    }
}

fn load_materials<R: BufRead + Seek>(
    obj: &mut Obj<SimplePolygon>,
    zip: &mut ZipArchive<R>,
    factory: &mut gfx_device_gl::Factory,
) -> HashMap<String, gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>> {
    use std::borrow::Cow;
    use std::io::BufReader;

    let mut materials = HashMap::new();

    for m in &obj.material_libs {
        let file = zip.by_name(m).unwrap();
        let mtl = Mtl::load(&mut BufReader::new(file));
        for m in mtl.materials {
            materials.insert(m.name.clone(), Cow::from(m));
        }
    }

    for object in &mut obj.objects {
        for group in &mut object.groups {
            if let Some(ref mut mat) = group.material {
                match materials.get(&mat.name) {
                    Some(newmat) => *mat = newmat.clone(),
                    None => {}
                };
            }
        }
    }

    let mut tex_files = HashSet::new();

    for m in materials.values() {
        tex_files.extend(
            vec![m.map_ka.as_ref(), m.map_kd.as_ref(), m.map_ks.as_ref()]
                .iter()
                .flat_map(|m| m.iter().cloned()),
        );
    }

    tex_files
        .into_iter()
        .map(|fname| {
            let texture = texture_from_zip(&fname, zip, factory).unwrap();
            (fname.to_owned(), texture)
        })
        .collect::<HashMap<_, _>>()
}

fn texture_from_zip<B: BufRead + Seek>(
    fname: &str,
    zip: &mut ZipArchive<B>,
    factory: &mut gfx_device_gl::Factory,
) -> Result<
    gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    Box<dyn std::error::Error>,
> {
    let mut buffer = vec![];
    zip.by_name(fname)?.read_to_end(&mut buffer)?;
    let image = ::image::load_from_memory(&buffer)?;
    let tbuf = super::load_texture(factory, &image);

    Ok(tbuf)
}

impl super::Drawable for Mario {
    fn draw(&self, window: &mut PistonWindow) {
        self.groups.iter().foreach(|m, _| m.draw(window));
    }
}
