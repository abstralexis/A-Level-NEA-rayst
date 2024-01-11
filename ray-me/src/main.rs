use notan::draw::{CreateDraw, DrawShapes, DrawConfig};
use notan::egui::{self, *};
use notan::math::Vec3;
use notan::math::Vec3Swizzles;
use notan::prelude::*;
use core::level_geometry::geometry::{Seg, Line};
use serde_json;
use std::fs::{write, read};
use std::io::BufReader;
use core::level_geometry::partitioning::non_recursive_partition;

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
    lines: Vec<Seg>,
    changes_saved: bool,
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
            lines: vec![],
            changes_saved: false,
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
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut line_renderer = gfx.create_draw();
    for line in &state.lines {
        let p1 = line.points().0.xy();
        let p2 = line.points().1.xy();
        line_renderer.line(p1.into(), p2.into()).color(Color::WHITE).width(2.0);
    }
    
    let mut output = plugins.egui(|ctx| {
        menu_bar(&ctx, state);
        tools_window(&ctx, state);
        if state.line_editor_open {
            line_editor_window(&ctx, state)
        }
        help_window(&ctx);
        assets_window(&ctx, state);
    });

    line_renderer.clear(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&line_renderer);
        gfx.render(&output);
    }
}

fn menu_bar(ctx: &Context, state: &mut State) {
    egui::TopBottomPanel::top("File Options Banner")
    .resizable(false)
    .exact_height(20.0)
    .show(&ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
            ui.menu_button("File", |ui| {
                ui.button("New");
                if ui.button("Save").clicked() {
                    let file_dialog = rfd::FileDialog::new();
                    let path = file_dialog.save_file();
                    match path {
                        None => {
                            egui::Window::new("Error E001")
                                .collapsible(false)
                                .resizable(false)
                                .show(&ctx, |ui| {
                                    ui.colored_label(Color32::RED, "An error occured getting path.");
                                });
                        },
                        Some(path) => {
                            let json = serde_json::to_string_pretty(&state.lines).unwrap();
                            match write(path, json) {
                                Ok(_) => (),
                                Err(_) => {egui::Window::new("Error E002")
                                    .collapsible(false)
                                    .resizable(false)
                                    .show(&ctx, |ui| {
                                        ui.colored_label(Color32::RED, "File save failed")
                                    });},
                            }
                        }
                    }
                };
                if ui.button("Open").clicked() {
                    let file_dialog = rfd::FileDialog::new();
                    let path = file_dialog.pick_file();
                    match path {
                        None => {
                            egui::Window::new("Error E003")
                                .collapsible(false)
                                .resizable(false)
                                .show(&ctx, |ui| {
                                    ui.colored_label(Color32::RED, "File select failed")
                                });
                        },
                        Some(path) => {
                            match read(path) {
                                Ok(data) => {
                                    let reader = BufReader::new(data.as_slice());
                                    let segs: Vec<Seg> = serde_json::from_reader(reader).unwrap();
                                    state.lines = segs;
                                },
                                Err(_) => {
                                    egui::Window::new("Error E003")
                                        .collapsible(false)
                                        .resizable(false)
                                        .show(&ctx, |ui| {
                                            ui.colored_label(Color32::RED, "File load failed")
                                        });
                                }
                            }
                        }
                    }
                };
            });

            ui.menu_button("Compile", |ui| {
                if ui.button("Compile").clicked() {
                    let file_dialog = rfd::FileDialog::new();
                    let path = file_dialog.save_file();
                    match path {
                        None => {
                            egui::Window::new("Error E001")
                                .collapsible(false)
                                .resizable(false)
                                .show(&ctx, |ui| {
                                    ui.colored_label(Color32::RED, "An error occured getting path.");
                                });
                        },
                        Some(path) => {
                            ui.spinner();
                            let partitioned = non_recursive_partition(state.lines.clone());
                            println!("{:?}", &partitioned); // dbg
                            let json = serde_json::to_string_pretty(&partitioned);
                            println!("{:?}", &json); // dbg
                            match write(path, json.unwrap()) {
                                Ok(_) => (),
                                Err(_) => {
                                    egui::Window::new("Error E004")
                                        .collapsible(false)
                                        .resizable(false)
                                        .show(&ctx, |ui| {
                                            ui.colored_label(Color32::RED, "Compilation error")
                                        });
                                }
                            }
                        }
                    }
                };
            });
        });
    });
}

fn tools_window(ctx: &Context, state: &mut State) {
    egui::Window::new("Tools")
    .anchor(Align2::LEFT_TOP, [0.0, 0.0])
    .default_width(200.0)
    .resizable(true)
    .show(&ctx, |ui| {
        ui.label("This window lets you select the tools you need for making a map.");
        ui.separator();

        if ui.button("⬈").on_hover_text("Line Drawer Parametric Interface").clicked() {
            state.line_editor_open = match state.line_editor_open {
                true => false,
                false => true,
            };
        }
    });
}

fn line_editor_window(ctx: &Context, state: &mut State) {
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

        let is_invalid_line = {
            state.l1_x.parse::<f32>().is_err() |
            state.l1_y.parse::<f32>().is_err() |
            state.l2_x.parse::<f32>().is_err() |
            state.l2_y.parse::<f32>().is_err() |
            state.line_bottom.parse::<f32>().is_err() |
            state.line_top.parse::<f32>().is_err()
        };

        let mut button_enabled = true; 
        if is_invalid_line {
            button_enabled = false;
            ui.colored_label(Color32::RED, "Please amend the above fields");
        }

        if ui.add_enabled(
            button_enabled, 
            egui::Button::new("Draw Line")
        ).clicked() { 
            let fl1_x = state.l1_x.parse::<f32>().unwrap();
            let fl1_y = state.l1_y.parse::<f32>().unwrap();
            let fl2_x = state.l2_x.parse::<f32>().unwrap();
            let fl2_y = state.l2_y.parse::<f32>().unwrap();
            let fl_bottom = state.line_bottom.parse::<f32>().unwrap();
            let fl_top = state.line_top.parse::<f32>().unwrap();

            let line = Seg::new(
                (
                    Vec3::from((fl1_x, fl1_y, fl_bottom)),
                    Vec3::from((fl2_x, fl2_y, fl_bottom))
                ), 
                fl_top - fl_bottom
            );

            state.lines.push(line);
        }
    });
}

fn help_window(ctx: &Context) {
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
}

fn assets_window(ctx: &Context, state: &mut State) {
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
}