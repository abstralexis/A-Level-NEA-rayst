use notan::egui::{self, *};
use notan::prelude::*;
use core::textures::TextureLoader;

#[derive(AppState)]
struct State {
    tools_open: bool,
}
impl State {
    pub fn init() -> Self {
        State {
            tools_open: true
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_vsync(true)
        .set_title("RayME - The Rayst Map Editor")
        .set_lazy_loop(true)
        .set_high_dpi(true);

    notan::init_with(State::init)
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| {
        // let missingtex = gfx.create_texture()
        //     .from_image(include_bytes!("../../core/textures/src/assets/missingtexture.png"))
        //     .build().unwrap();
        let loader = TextureLoader::new(gfx);
        let missingtex = loader.textures.get("missingtex").unwrap();
        let s_missingtex = gfx.egui_register_texture(&missingtex);

        egui::Window::new("Tools")
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .default_width(200.0)
            .resizable(true)
            .show(&ctx, |ui| {
                ui.label("This window lets you select the tools you need for making a map.");
                ui.separator();
                ui.image(s_missingtex);
            });

        egui::Window::new("Help")
            .anchor(Align2::LEFT_BOTTOM, [0.0, 0.0])
            .default_width(200.0)
            .resizable(true)
            .show(&ctx, |ui| {
                ui.label("This is the help section of the editor. Use me if you get stuck!");
                ui.separator();

                use notan::egui::special_emojis::GITHUB;
                ui.hyperlink_to(
                    format!("{GITHUB} Github"), 
                    "https://github.com/AlexisComix/A-Level-NEA"
                );
            });

        egui::Window::new("Assets")
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .default_width(200.0)
            .resizable(true)
            .show(&ctx, |ui| {
                ui.label("This is the area where you can select assets and put them down.");
                ui.separator();
                ui.collapsing("Behaviour", |ui| {
                    ui.label("This is the section where the Entity Behaviour Code is.");
                });
                ui.collapsing("Textures", |ui| {
                    ui.label("This is where the textures library is.");
                });
                ui.collapsing("Entities", |ui| {
                    ui.label("This is where the Entity Parts are put together.");
                    ui.separator();
                    ui.button("Create New");
                });
                ui.collapsing("Audio", |_| {});
            });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}