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
pub struct Object {
    groups: Vec<super::ObjectData>,
    start_time: std::time::SystemTime,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    time_offset: f32,
}

impl Object {
    pub fn load(
        url: &str,
        pipeline: Arc<Mutex<crate::pipeline::ObjectPipeline>>,
        factory: &mut gfx_device_gl::Factory,
    ) -> Result<Object, Box<dyn std::error::Error>> {
        use std::io::BufReader;

        let mut zip_archive = {
            let model_file = BufReader::new(super::download_cached(url)?);
            ZipArchive::new(model_file)?
        };

        let mut obj = {
            let obj_file_name = (0..zip_archive.len())
                .into_iter()
                .filter_map(|i| match zip_archive.by_index(i) {
                    Ok(f) => Some(f.name().to_owned()),
                    _ => None,
                })
                .filter(|f| f.ends_with(".obj"))
                .next()
                .ok_or("Missing .obj file in archive")?;
            let obj_file = zip_archive.by_name(&obj_file_name)?;
            Obj::<SimplePolygon>::load_buf(&mut BufReader::new(obj_file))?
        };

        let textures = load_materials(&mut obj, &mut zip_archive, factory)?;

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
            .filter_map(|g| {
                let indices: Vec<u16> = g
                    .polys
                    .iter()
                    .flat_map(|p| p.iter().map(|t| t.0 as u16))
                    .collect();
                let tex_name = g.material.as_ref()?.map_kd.as_ref()?;
                Some(super::ObjectData::new(
                    pipeline.clone(),
                    factory,
                    vbuf.clone(),
                    &indices,
                    textures[tex_name].clone(),
                ))
            })
            .collect();
        drop(textures);

        Ok(Object {
            groups,
            start_time: std::time::SystemTime::now(),
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            time_offset: 0.0,
        })
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
}

fn load_materials<R: BufRead + Seek>(
    obj: &mut Obj<SimplePolygon>,
    zip: &mut ZipArchive<R>,
    factory: &mut gfx_device_gl::Factory,
) -> Result<
    HashMap<String, gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>>,
    Box<dyn std::error::Error>,
> {
    use std::borrow::Cow;
    use std::io::BufReader;

    let mut materials = HashMap::new();

    for m in &obj.material_libs {
        let file = zip.by_name(m)?;
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

    let textures = tex_files
        .into_iter()
        .map(|fname| {
            let texture = texture_from_zip(&fname, zip, factory)?;
            Ok((fname.to_owned(), texture))
        })
        .try_fold(
            HashMap::new(),
            |mut acc, t: std::result::Result<_, Box<dyn std::error::Error>>| {
                let (k, v) = t?;
                acc.insert(k, v);
                Ok(acc)
            },
        );

    textures
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
    let tbuf = super::load_texture(factory, &image)?;

    Ok(tbuf)
}

impl super::Drawable for Object {
    fn draw(&self, window: &mut PistonWindow) {
        self.groups.iter().foreach(|m, _| m.draw(window));
    }
}
