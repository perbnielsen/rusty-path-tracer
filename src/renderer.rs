use crate::{
    camera::Camera,
    colour::{Colour, BLACK},
    ray::Ray,
    scene::Scene,
};
use scoped_threadpool::Pool;

pub struct Renderer {
    pub num_workers: usize,
    pub num_chunks: usize,
    pub scene: Scene,
}

// Keep an eye on this, for later optimizations:
// https://doc.rust-lang.org/alloc/slice/struct.ArrayChunksMut.html

impl Renderer {
    pub fn render(&self, camera: &Camera, width: usize, height: usize) -> Vec<Colour> {
        let image_size = width * height;
        let chunk_size = image_size / self.num_chunks;
        let mut image = vec![BLACK; image_size];
        let rays: Vec<Ray> = camera.get_viewport(width, height).collect();

        Pool::new(self.num_workers as u32).scoped(|scope| {
            let ray_chunks = rays.chunks(chunk_size);
            let image_chunks = image.chunks_mut(chunk_size);

            let jobs = ray_chunks.zip(image_chunks);
            for (rays, image_chunk) in jobs {
                scope.execute(move || {
                    rays.iter()
                        .enumerate()
                        .for_each(|(idx, ray)| image_chunk[idx] = self.scene.cast_ray(ray, 0));
                });
            }
        });

        image
    }
}
