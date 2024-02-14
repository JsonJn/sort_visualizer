// use crate::font::Font;
use crate::manager::State;
use crate::sort_visualizer::sort_executor::SortExecutor;
use crate::sort_visualizer::sorts::Sorts;
use crate::sort_visualizer::structures::SortList;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl_utils::LoadedFont;

mod sort_executor;

mod sorts;
mod structures;

pub struct SortVisualizer {
    width: u32,
    height: u32,

    length: usize,
    sort_exec: SortExecutor,

    sorting: bool,

    selected_sort: usize,

    font: LoadedFont,
}

impl State for SortVisualizer {
    fn new(width: u32, height: u32) -> Self {
        let length = 2048;
        // let rand_list = SortList::create_rand_list(length);
        let mut rand_list = SortList::create_list(length);
        rand_list.reverse();
        Self {
            width,
            height,
            length,
            sort_exec: SortExecutor::new(rand_list),
            font: LoadedFont::default(),
            sorting: false,
            selected_sort: Sorts::Smooth.index_of(),
        }
    }

    fn process_event(&mut self, event: Event) {
        #[allow(clippy::collapsible_match, clippy::single_match)]
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                keymod,
                ..
            } => match keycode {
                Keycode::S => {
                    self.sort_exec
                        .sort_and_replace(Sorts::VARIANTS[self.selected_sort]);
                }
                Keycode::Space => {
                    self.sorting = !self.sorting;
                }
                Keycode::R => {
                    if keymod.contains(Mod::LSHIFTMOD) {
                        let mut data = SortList::create_list(self.length);
                        data.reverse();
                        self.sort_exec.set_data(data);
                    } else if keymod.contains(Mod::LCTRLMOD) {
                        let mut data = SortList::create_list(self.length);
                        let mut rng = rand::thread_rng();
                        for _ in 0..2 {
                            let (i1, i2) =
                                (rng.gen_range(0..data.len()), rng.gen_range(0..data.len()));
                            data.swap(i1, i2);
                        }
                        self.sort_exec.set_data(data);
                    } else {
                        self.sort_exec
                            .set_data(SortList::create_rand_list(self.length));
                    }
                }
                Keycode::Left => {
                    self.selected_sort = if self.selected_sort == 0 {
                        Sorts::VARIANTS.len() - 1
                    } else {
                        self.selected_sort - 1
                    }
                }
                Keycode::Right => {
                    self.selected_sort = (self.selected_sort + 1) % Sorts::VARIANTS.len()
                }
                // (Keycode::Delete, _) => {
                //     for op in &self.sort_ops {
                //         println!("{:?}", op);
                //     }
                // }
                _ => {}
            },
            _ => {}
        }
    }

    fn tick(&mut self, window_canvas: &mut WindowCanvas) {
        if self.sorting {
            for _ in 0..400 {
                self.sort_exec.execute_one();
            }
            if self.sort_exec.out_of_ops() {
                self.sorting = false;
            }
        }

        let length = self.length as u32;
        let width_per = self.width / length;
        let height_per = (self.height / length) as f32;
        let height_per = if height_per == 0.0 {
            self.height as f32 / length as f32
        } else {
            height_per
        };
        let x_offset = (self.width - width_per * length) / 2;

        let (highlight, highlight_aux) = self.sort_exec.take_and_clear_highlights();
        let data = self.sort_exec.get_data();

        for i in 0..self.length {
            let x = width_per * i as u32 + x_offset;
            let val = data[i];
            let height = (val as f32 * height_per) as u32;
            let rect = Rect::new(
                x as i32,
                self.height as i32 - height as i32,
                width_per,
                height,
            );
            window_canvas.set_draw_color(if highlight.contains(&i) {
                Color::RED
            } else {
                Color::WHITE
            });
            window_canvas.fill_rect(rect).unwrap();
        }
        let mut displayed = String::new();
        displayed.push_str(&format!(
            "Sort: {}\n",
            Sorts::VARIANTS[self.selected_sort].name()
        ));

        let sort_ops = self.sort_exec.get_sort_ops();
        if self.sorting {
            displayed.push_str("Sorting...\n");
        } else {
            displayed.push_str("Sort ready.\n");
        };

        displayed.push_str(&format!("Stored operations: {}\n", sort_ops.len()));
        displayed.push_str(&format!(
            "Current operation: {}\n",
            self.sort_exec.get_index()
        ));
        displayed.push_str(&format!("Next operation: {:?}\n", self.sort_exec.next_op()));

        window_canvas.set_draw_color(Color::GRAY);
        self.font.draw(window_canvas, displayed.chars(), 2, 5, 5);
    }
}
