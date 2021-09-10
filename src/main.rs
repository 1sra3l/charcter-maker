mod mainview;
mod data;
mod ini;

use crate::data::Stats;
use crate::ini::IniDetails;

// GUI
use fltk::{prelude::*, image::*, *};

use std::fs;

extern crate tini;
use tini::Ini;


#[derive (Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Save,
    Load,
    Quit,
    Help,
    SpriteSheet,
    Switch,
}

fn make_image(file_name:String) -> Result<SharedImage, FltkError> {
    if file_name == "" {
        return Err(FltkError::Unknown("Empty file name".to_string()))
    }
    let filename = match fs::canonicalize(file_name.to_owned()) {
        Ok(filename) => filename.to_str().unwrap().to_owned(),
        Err(e) => {
            println!("make_image`canonicalize` ERROR: {:?}\nLikely due to the file: {:?}\nTry using an absolute path, if encountering errors", e, file_name);
            return Err(FltkError::IoError(e))
        },
    };
    fltk::image::SharedImage::load(filename.to_owned().as_str())
}


fn main() {
    let app = app::App::default();
    let mut ui = mainview::UI::make_window();
    let mut stats:Stats = Stats::empty();
    let (send_action, receive_action) = app::channel::<Action>();

    ui.quit.emit(send_action, Action::Quit);
    ui.load.emit(send_action, Action::Load);
    ui.save.emit(send_action, Action::Save);
    ui.help.emit(send_action, Action::Help);
    ui.sprite_sheet.emit(send_action, Action::SpriteSheet);
    ui.characters.emit(send_action, Action::Switch);

    //run the app
    while app.wait() {
        // Check the buttons
        stats = Stats{
            hp:ui.hp.value(),
            mp:ui.mp.value(),
            atk:ui.atk.value(),
            def:ui.def.value(),
            m_atk:ui.m_atk.value(),
            m_def:ui.m_def.value(),
            gp:ui.gp.value(),
            xp:ui.xp.value(),
            xp_next:ui.xp_next.value(),
            mp_max:ui.mp.value(),
            hp_max:ui.hp.value(),
            level:ui.level.value(),
            speed:ui.speed.value(),
            name:ui.name.value().to_owned(),
            image:ui.sprite_sheet.label().to_owned(),
            class:ui.class.label().to_owned(),
            c_type:ui.c_type.label().to_owned(),
            ini_details:stats.ini_details.clone(),
        };
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Action::Save => {
                    let file = match dialog::file_chooser("Choose File", "*.ini", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    stats.save(file.to_string());
                },
                Action::Load => {
                    let file = match dialog::file_chooser("Choose File", "*.ini", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    ui.file.set_value(file.as_str());
                    let details = IniDetails::load(file.to_owned(), "default");
                    let test_ini = Ini::from_string(details.ini_string.to_owned());
                    if test_ini.is_err() {
                        println!("ERROR reading the file:{:?}", test_ini.err());
                        continue;
                    }
                    ui.characters.clear();
                    ui.sprite_sheet.set_image::<SharedImage>(None);
                    let conf = test_ini.unwrap();
                    for (name, _section_iter) in conf.iter() {     
                        ui.characters.add(name.as_str());
                    }
                    
                },
                Action::Switch => {
                    ui.sprite_sheet.set_image::<SharedImage>(None);
                    let section = match ui.characters.selected_text(){
                        Some(section) => section,
                        None => continue,
                    };
                    println!("Switch to:{:?}", section.to_owned());
                    let details = IniDetails::load(ui.file.value().to_owned(), section.to_owned().as_str());
                    stats = Stats::load(details);
                    ui.hp.set_value(stats.hp);
                    ui.mp.set_value(stats.mp);
                    ui.xp.set_value(stats.xp);
                    ui.gp.set_value(stats.gp);
                    ui.atk.set_value(stats.atk);
                    ui.def.set_value(stats.def);
                    ui.m_atk.set_value(stats.m_atk);
                    ui.m_def.set_value(stats.m_def);
                    ui.speed.set_value(stats.speed);
                    ui.name.set_value(stats.name.as_str());
                    ui.class.set_label(stats.class.as_str());
                    ui.c_type.set_label(stats.c_type.as_str());
                    ui.sprite_sheet.set_label(stats.image.as_str());
                    let img = make_image(stats.image.to_owned());
                    if img.is_ok() {
                        let img = img.ok().unwrap();
                        ui.sprite_sheet.set_image(Some(img.to_owned()));
                    }
                    ui.win.redraw();
                },
                Action::Quit => app::quit(),
                Action::Help => (),
                Action::SpriteSheet => {
                    let file = match dialog::file_chooser("Choose an Image", "*.png, *.svg, *.jpg, *.gif", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    let img = make_image(file.to_owned());
                    if img.is_ok() {
                        let img = img.ok().unwrap();
                        ui.sprite_sheet.set_image(Some(img.to_owned()));
                        ui.sprite_sheet.set_label(file.to_owned().as_str());
                    }
                    ui.win.redraw();
                },
            }
        }
        ui.win.redraw();

    }
}
