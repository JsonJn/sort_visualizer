// use crate::font::MinecraftFont;
use crate::manager::run;
use crate::sort_visualizer::SortVisualizer;

// mod font;
mod manager;
mod sort_visualizer;

fn main() {
    run::<SortVisualizer>(2048, 1200);
}
