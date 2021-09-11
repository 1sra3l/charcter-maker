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
    Story,
    New,
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
    let menu = ui.menu.clone();
    ui.menu.add(
    "&File/&New...\t",
    enums::Shortcut::Ctrl | 'q',
    menu::MenuFlag::Normal,
     |_| (),);
    ui.menu.add(
    "&File/&Open...\t",
    enums::Shortcut::Ctrl | 'o',
    menu::MenuFlag::Normal,
     |_| (),);
    ui.menu.add(
    "&File/&Save...\t",
    enums::Shortcut::Ctrl | 's',
    menu::MenuFlag::Normal,
     |_| (),);
    ui.menu.add(
    "&File/&Quit...\t",
    enums::Shortcut::Ctrl | 'q',
    menu::MenuFlag::Normal,
     |_| (),);
    let mut m = match menu.at(1){
        Some(m) => m,
        None => return,
    };
    m.emit(send_action, Action::Load);
    m = match menu.at(2){
        Some(m) => m,
        None => return,
    };
    m.emit(send_action, Action::Load);
    m = match menu.at(3){
        Some(m) => m,
        None => return,
    };
    m.emit(send_action, Action::Save);
    m = match menu.at(4){
        Some(m) => m,
        None => return,
    };
    m.emit(send_action, Action::Quit);
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
            agility:0.0,
            strength:0.0,
            dexterity:0.0,
            constitution:0.0,
            intelligence:0.0,
            charisma:0.0,
            wisdom:0.0,
            age:0.0,
            name:ui.name.value().to_owned(),
            image:ui.sprite_sheet.label().to_owned(),
            class:ui.class.label().to_owned(),
            c_type:ui.c_type.label().to_owned(),
            ini_details:stats.ini_details.clone(),
            clan:String::from(""),
            m_weak:String::from(""),
            m_strong:String::from(""),
            m_attacks:vec![],
            m_type:String::from(""),
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
                Action::Story => {
                    let file = match dialog::file_chooser("Choose File", "*.txt", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    ui.story.set_value(file.as_str());
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
                Action::New => {
                    ui.sprite_sheet.set_image::<SharedImage>(None);
                    ui.hp.set_value(0.0);
                    ui.mp.set_value(0.0);
                    ui.xp.set_value(0.0);
                    ui.gp.set_value(0.0);
                    ui.atk.set_value(0.0);
                    ui.def.set_value(0.0);
                    ui.m_atk.set_value(0.0);
                    ui.m_def.set_value(0.0);
                    ui.speed.set_value(0.0);
                    ui.agility.set_value(0.0);
                    ui.strength.set_value(0.0);
                    ui.dexterity.set_value(0.0);
                    ui.constitution.set_value(0.0);
                    ui.intelligence.set_value(0.0);
                    ui.charisma.set_value(0.0);
                    ui.wisdom.set_value(0.0);
                    ui.level.set_value(0.0);
                    ui.age.set_value(0.0);
                    ui.name.set_value("");
                    ui.class.set_label("Class");
                    ui.clan.set_label("Clan");
                    ui.c_type.set_label("Type");
                    ui.m_strong.set_label("M Strong");
                    ui.m_weak.set_label("M Weak");
                    ui.m_type.set_label("M Type");
                    ui.bonus.set_label("Bonus");
                    ui.stage.set_label("Stage");
                    ui.sprite_sheet.set_label("Sprite Sheet");
                    ui.story.set_value("");
                    ui.file.set_value("");
                    ui.win.redraw();
                },
                Action::Switch => {
                    ui.sprite_sheet.set_image::<SharedImage>(None);
                    let section = match ui.characters.selected_text(){
                        Some(section) => section,
                        None => continue,
                    };
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
                    
                    ui.agility.set_value(0.0);
                    ui.strength.set_value(0.0);
                    ui.dexterity.set_value(0.0);
                    ui.constitution.set_value(0.0);
                    ui.intelligence.set_value(0.0);
                    ui.charisma.set_value(0.0);
                    ui.wisdom.set_value(0.0);
                    ui.level.set_value(0.0);
                    ui.age.set_value(0.0);
                    ui.name.set_value("");
                    ui.class.set_label(stats.class.as_str());
                    ui.c_type.set_label(stats.c_type.as_str());
                    ui.clan.set_label("Clan");
                    ui.m_strong.set_label("M Strong");
                    ui.m_weak.set_label("M Weak");
                    ui.m_type.set_label("M Type");
                    ui.bonus.set_label("Bonus");
                    ui.stage.set_label("Stage");
                    ui.sprite_sheet.set_label(stats.image.as_str());
                    ui.story.set_value("");
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
