use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A very simple ray-tracer")]
pub struct CommandLineOptions {
    /// Json-file containing the scene to render
    #[structopt(default_value = "scene.json")]
    pub scene: String,

    /// Width of the render in pixels
    #[structopt(default_value = "1920")]
    pub width: usize,

    /// Height of the render in pixels
    #[structopt(default_value = "1024")]
    pub height: usize,
}
