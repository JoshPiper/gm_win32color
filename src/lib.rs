#![feature(c_unwind)]

use gmod::lua::{State};
use gmod::lua_function;
use ansi_term::{Colour, Style};
use std::io::{stdout, Write};

#[macro_use] extern crate gmod;

static MOD_NAME: &str = "win32color";
macro_rules! err {
    () => {format!("{} had an error.", MOD_NAME)};
    ($arg:literal) => {format!("{} was unable to {}", MOD_NAME, $arg)};
    ($arg:literal, $err:literal) => {format!("{} was unable to {}: {:?}", MOD_NAME, $arg, $err)};
}

unsafe fn error<S: AsRef<str>>(lua: State, err: S){
    lua.error(err);
}

#[lua_function]
unsafe fn test(lua: State){
    let cols = termsize::get().unwrap().cols;

    for col in 0..cols {
        print!("{}", Style::default().fg(Colour::RGB(((col * 255) / cols).try_into().unwrap(), 0, 0)).paint("|"));
    }
    println!();

    for col in 0..cols {
        print!("{}", Style::default().fg(Colour::RGB(0, ((col * 255) / cols).try_into().unwrap(), 0)).paint("|"));
    }
    println!();

    for col in 0..cols {
        print!("{}", Style::default().fg(Colour::RGB(0, 0, ((col * 255) / cols).try_into().unwrap())).paint("|"));
    }
    println!();
}

#[lua_function]
unsafe fn truecolor_print(lua: State){

    let r = lua.check_number(1).round() as u8;
    let g = lua.check_number(2).round() as u8;
    let b = lua.check_number(3).round() as u8;
    let str = lua.check_string(4);

    print!("{}\r", Style::default().fg(Colour::RGB(r, g, b)).paint(str));
    stdout().flush().expect("");
    0
}

#[lua_function]
unsafe fn get_console_size(lua: State){
    let size = termsize::get().unwrap();
    lua.push_number(size.cols.into());
    lua.push_number(size.rows.into());
    2
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.sysinfo.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        }
    }

    lua.create_table(0, 8);
    export_lua_function!(truecolor_print);
    export_lua_function!(test);
    export_lua_function!(get_console_size);
    lua.set_global(lua_string!("win32color"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
