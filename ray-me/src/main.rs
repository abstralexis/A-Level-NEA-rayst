use notan::egui::{self, *};
use notan::prelude::*;
use core::textures::TextureLoader;
use std::fmt::format;

#[derive(AppState)]
struct State {
    tools_open: bool,
    l1_x: String,
    l1_y: String,
    l2_x: String,
    l2_y: String,
    line_bottom: String,
    line_top: String,
    line_editor_open: bool,
}
impl State {
    pub fn init() -> Self {
        State {
            tools_open: true,
            l1_x: "0.0".to_owned(),
            l1_y: "0.0".to_owned(),
            l2_x: "0.0".to_owned(),
            l2_y: "0.0".to_owned(),
            line_bottom: "0.0".to_owned(),
            line_top: "200.0".to_owned(),
            line_editor_open: false,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let logo = include_bytes!("../../core/textures/src/assets/rayme_logo.png");

    let win = WindowConfig::new()
        .set_vsync(true)
        .set_title("RayME - The Rayst Map Editor")
        .set_window_icon_data(Some(logo))
        .set_taskbar_icon_data(Some(logo))
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

                if ui.button("⬈").on_hover_text("Line Drawer Parametric Interface").clicked() {
                    state.line_editor_open = match state.line_editor_open {
                        true => false,
                        false => true,
                    };
                }

            });

        if state.line_editor_open{
            egui::Window::new("Parametric Line Drawer")
                .default_width(200.0)
                .resizable(false)
                .show(&ctx, |ui| {
                    ui.label("This window lets you specify line coordinates then draw them to the screen.");
                    ui.separator();

                    ui.label("Line first point x");
                    ui.text_edit_singleline(&mut state.l1_x);
                    if state.l1_x.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    ui.label("Line first point y");
                    ui.text_edit_singleline(&mut state.l1_y);
                    if state.l1_y.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    ui.label("Line second point x");
                    ui.text_edit_singleline(&mut state.l2_x);
                    if state.l2_x.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    ui.label("Line second point y");
                    ui.text_edit_singleline(&mut state.l2_y);
                    if state.l2_y.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    ui.label("Line bottom coordinate");
                    ui.text_edit_singleline(&mut state.line_bottom);
                    if state.line_bottom.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    ui.label("Line top coordinate");
                    ui.text_edit_singleline(&mut state.line_top);
                    if state.line_top.parse::<f32>().is_err() {
                        ui.colored_label(Color32::RED, "Coordinate must be a numeric value.");
                    }

                    let mut button_enabled = true; 
                    if {
                        state.l1_x.parse::<f32>().is_err() |
                        state.l1_y.parse::<f32>().is_err() |
                        state.l2_x.parse::<f32>().is_err() |
                        state.l2_y.parse::<f32>().is_err() |
                        state.line_bottom.parse::<f32>().is_err() |
                        state.line_top.parse::<f32>(). is_err()
                    } {
                        button_enabled = false;
                        ui.colored_label(Color32::RED, "Please amend the above fields");
                    }

                    ui.add_enabled(button_enabled, egui::Button::new("Draw Line"));
                });
        }

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