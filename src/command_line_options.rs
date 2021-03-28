use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A very simple ray-tracer")]
pub struct CommandLineOptions {
    /// Json-file containing the scene to render
    #[structopt(default_value = "scene.json", short, long)]
    pub scene: String,

    /// Width of the render in pixels
    #[structopt(default_value = "1920")]
    pub width: usize,

    /// Height of the render in pixels
    #[structopt(default_value = "1024")]
    pub height: usize,

    ///Number of worker threads to spawn
    #[structopt(default_value = "1", short, long)]
    pub num_workers: usize,

    ///Number of chunks to split the image into
    #[structopt(default_value = "100", long)]
    pub num_chunks: usize,

    ///Name of output image
    #[structopt(default_value = "image.ppm", long)]
    pub image_name: String,
}
