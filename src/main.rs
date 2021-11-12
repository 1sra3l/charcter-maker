mod mainview;

// GUI
use fltk::{prelude::*, image::*, *};

//use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::fs;
use std::io::Read;
use std::fs::File;
use std::io::Write;

use rpgstat::stats::Advanced as Stats;
//use rpgstat::stats::{Builder, Normal, Basic};
//extern crate num;

use toml::*;
use serde::{Deserialize, Serialize};

#[derive (Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Save,
    Load,
    Quit,
    Help,
    SpriteSheet,
    New,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub name:String,
    pub image:String,
    pub stats:Stats<f64>,
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
    let stats:Stats<f64> = Stats::empty::<f64>();
    let mut character = Character {
        name:String::from(""),
        image:String::from(""),
        stats:stats,
    }; 
    let (send_action, receive_action) = app::channel::<Action>();
    let menu = ui.menu.clone();

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
    //run the app
    while app.wait() {
        // Check the buttons
        character.stats = Stats {
            id:0.0,//TODO
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
            agility:ui.agility.value(),
            strength:ui.strength.value(),
            dexterity:ui.dexterity.value(),
            constitution:ui.constitution.value(),
            intelligence:ui.intelligence.value(),
            charisma:ui.charisma.value(),
            wisdom:ui.wisdom.value(),
            age:ui.age.value(),
        };
            character.name = ui.name.value().to_owned();
            character.image = ui.sprite_sheet.label().to_owned();
//TODO?
            //class:ui.class.label().to_owned(),
            //c_type:ui.c_type.label().to_owned(),

        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Action::Save => {
                    let file = match dialog::file_chooser("Choose File", "Configuration Files (*.{toml,ini})\t*.toml\t*.ini\t*.", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    let mut filename:String = match fs::canonicalize(file.to_owned()) {
                        Ok(filename) => {
                            filename.to_str().unwrap().to_owned()
                        },
                        Err(e) => {
                            println!("`fs::canonicalize` ERROR: {:?} file: {:?}", e, file);
                            continue
                        },
                    };
                    let toml = match toml::to_string(&character){
                        Ok(toml) => toml,
                        Err(e) => {
                            println!("`toml::to_string` ERROR: {:?} charcter: {:?}", e, character);
                            continue
                        },
                    };
                    let mut filename = match File::create(file.to_owned()){
                        Ok(filename) => filename,
                        Err(e) => {
                            println!("`File::create` ERROR: {:?} file: {:?}", e, file.to_owned());
                            continue
                        },
                    };
                    match filename.write_all(toml.to_owned().as_bytes()){
                        Ok(_) => {},
                        Err(e) => {
                            println!("`Write::write_all` ERROR: {:?} file: {:?}", e, file.to_owned())
                        }
                    };
                    match filename.sync_all(){
                        _=>{},
                    };
                    //character.save(file.to_string());//TODO
                },
                Action::Load => {
                    let file = match dialog::file_chooser("Choose File", "Configuration Files (*.{toml,ini})\t*.toml\t*.ini\t*.", ".", true) {
                        Some(file) => file,
                        None => continue,
                    };
                    let mut filename:String = match fs::canonicalize(file.to_owned()) {
                        Ok(filename) => {
                            filename.to_str().unwrap().to_owned()
                        },
                        Err(e) => {
                            println!("`fs::canonicalize` ERROR: {:?} file: {:?}", e, file);
                            continue
                        },
                    };
                    match File::open(filename.to_owned()) {
                        Ok(mut fname) => {
                            ui.file.set_value(filename.as_str());
                            let mut content = String::new();
                            match fname.read_to_string(&mut content){
                                Ok(..)=>{},
                                Err(e)=>{
                                    println!("`Read::read_to_string` ERROR: {:?} file: {:?}", e, fname);
                                    continue
                                },
                            };
                            character = match toml::from_str(&content.to_owned()){
                                Ok(c) => c,
                                Err(e) => {
                                    println!("`toml::from_str` ERROR: {:?} file: {:?}", e, fname);
                                    continue
                                },
                            };
                        },
                        Err(e) => {
                            println!("`File::open` ERROR: {:?} file: {:?}", e, filename);
                            continue
                        },
                    };
                    ui.sprite_sheet.set_image::<SharedImage>(None);
                    ui.name.set_value(character.name.as_str());
                    ui.hp.set_value(character.stats.hp);
                    ui.mp.set_value(character.stats.mp);
                    ui.xp.set_value(character.stats.xp);
                    ui.gp.set_value(character.stats.gp);
                    ui.atk.set_value(character.stats.atk);
                    ui.def.set_value(character.stats.def);
                    ui.m_atk.set_value(character.stats.m_atk);
                    ui.m_def.set_value(character.stats.m_def);
                    ui.speed.set_value(character.stats.speed);
                    ui.agility.set_value(character.stats.agility);
                    ui.strength.set_value(character.stats.strength);
                    ui.dexterity.set_value(character.stats.dexterity);
                    ui.constitution.set_value(character.stats.constitution);
                    ui.intelligence.set_value(character.stats.intelligence);
                    ui.charisma.set_value(character.stats.charisma);
                    ui.wisdom.set_value(character.stats.wisdom);
                    ui.level.set_value(character.stats.level);
                    ui.age.set_value(character.stats.age);
                    //ui.class.set_label(character.stats.class.as_str());
                    //ui.c_type.set_label(character.stats.c_type.as_str());
                    ui.sprite_sheet.set_label(character.image.as_str());
                    //ui.story.set_value("");
                    let img = make_image(character.image.to_owned());
                    if img.is_ok() {
                        let mut img = img.ok().unwrap();
                        img.scale(ui.sprite_sheet.w(), ui.sprite_sheet.h(), true, true);
                        ui.sprite_sheet.set_image(Some(img.to_owned()));
                    }
                    ui.win.redraw();
                    
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
                    ui.c_type.set_label("Type");
                    ui.stage.set_label("Stage");
                    ui.sprite_sheet.set_label("Sprite Sheet");
                    //ui.story.set_value("");
                    ui.file.set_value("");
                    ui.win.redraw();
                },
                Action::Quit => app::quit(),
                Action::Help => (),
                Action::SpriteSheet => {
                    let file = match dialog::file_chooser("Choose an Image", "Images (*.{png,svg,jpg,gif})\t*.", ".", true){
                        Some(file) => file,
                        None => continue,
                    };
                    let img = make_image(file.to_owned());
                    if img.is_ok() {
                        let mut img = img.ok().unwrap();
                        img.scale(ui.sprite_sheet.w(), ui.sprite_sheet.h(), true, true);
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
