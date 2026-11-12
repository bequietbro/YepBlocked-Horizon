// YepBlocked: a Horizon native mod made with Horizon Translator's mod kit.
// This file is both the mod's Cargo library (`cargo test` runs the tests at the bottom without the game)
// and the module the bundle adds to the game (crate::game::kit_yepblocked). The small files beside it (key.rs, hud.rs...)
// are the hooks: Horizon calls them, and they call the functions here. Change anything you like; the guide
// (Make a mod > Guide in Horizon Translator) says what each hook and game helper does.
// Module files can't hold `//!` comments or inner attributes: keep to plain `//` comments.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// The mod's id (its settings folder and its keys' lines), name, version and description.
pub const ID: &str = "yepblocked";
pub const NAME: &str = "YepBlocked";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Mod for detailed mob/animal spawn configuration";
pub const AUTHOR: &str = "BEQUIETBRO";

/// The mod's keys, as Options > Mods lists them: name, label, default key. Players can change them there.
pub const KEYS: &[[&str; 3]] = &[
];

/// What a key or a command does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Puts these lines in chat (placeholders filled in).
    Say(&'static [&'static str]),
    /// Shows or hides the HUD box.
    ToggleHud,
    /// Zooms in, or back out.
    ToggleZoom,
    /// Zooms while the key is held.
    HoldZoom,
    /// Lists the mod's commands and keys (the /yepblocked command).
    Help,
}

/// What each key (by its name in KEYS) does.
pub const KEY_ACTIONS: &[(&str, Action)] = &[
];

/// The mod's chat commands, without the slash, and what they do. /yepblocked (the help) is added by `command`.
pub const COMMANDS: &[(&str, Action)] = &[
    ("hello", Action::Say(&["Hello from {name}! You're at {bx}, {by}, {bz} in the {dimension}."])),
];

/// The HUD box: its corner (0 top left, 1 top right, 2 bottom left, 3 bottom right), its lines, the text colour
/// (0xRRGGBB) and the box behind it (0xAARRGGBB, 0 for none).
pub const HUD_CORNER: usize = 0;
pub const HUD_LINES: &[&str] = &[];
pub const HUD_INK: u32 = 0xFFFFFF;
pub const HUD_BACKGROUND: u32 = 0x7800_0000;

/// Zoom: how many times closer, eased like the spyglass (smooth), and whether the mouse wheel changes it while zooming.
pub const ZOOM_FACTOR: f32 = 4.0;
pub const ZOOM_SMOOTH: bool = true;
pub const ZOOM_SCROLL: bool = false;

/// Said in chat on joining a world.
pub const WELCOME: &[&str] = &[];

/// Said in chat every so many seconds while in a world.
pub const TIMERS: &[(u32, &[&str])] = &[
];

/// The ON/OFF options of the mod's settings page (Options > Mods > YepBlocked): key, label. All start ON.
pub const TOGGLES: &[(&str, &str)] = &[
];

// ------------------------------------------------------------------ text

/// What the text placeholders show: the player, in a world.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub dimension: String,
    pub fps: u32,
}

impl Info {
    /// From the game's `native_player()` (x, y, z, yaw, pitch, dimension) and frame rate.
    pub fn new(player: (f64, f64, f64, f32, f32, &str), fps: u32) -> Self {
        let (x, y, z, yaw, pitch, dimension) = player;
        Info { x, y, z, yaw, pitch, dimension: dimension.to_owned(), fps }
    }
}

/// The way the player faces (Minecraft's yaw: 0 faces south, 90 west).
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South (+Z)", "West (-X)", "North (-Z)", "East (+X)"][i as usize]
}

/// `minecraft:the_nether` as players say it.
pub fn dimension_name(id: &str) -> String {
    match id {
        "minecraft:overworld" => "Overworld".into(),
        "minecraft:the_nether" => "Nether".into(),
        "minecraft:the_end" => "End".into(),
        other => other.rsplit(':').next().unwrap_or(other).replace('_', " "),
    }
}

/// `text` with its placeholders ({x}, {facing}, {fps}...) filled in. Unknown ones stay as they are; outside a
/// world (`info` None) the ones about the player show `?`.
pub fn fill(text: &str, info: Option<&Info>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else {
            rest = &rest[open..];
            break;
        };
        let name = &after[..close];
        match value(name, info) {
            Some(v) => out.push_str(&v),
            None => {
                out.push('{');
                out.push_str(name);
                out.push('}');
            }
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

fn value(name: &str, info: Option<&Info>) -> Option<String> {
    match name {
        "name" => return Some(NAME.into()),
        "version" => return Some(VERSION.into()),
        _ => {}
    }
    let known = ["x", "y", "z", "bx", "by", "bz", "facing", "yaw", "pitch", "dimension", "fps"];
    if !known.contains(&name) {
        return None;
    }
    let Some(i) = info else {
        return Some("?".into());
    };
    Some(match name {
        "x" => format!("{:.1}", i.x),
        "y" => format!("{:.1}", i.y),
        "z" => format!("{:.1}", i.z),
        "bx" => format!("{}", i.x.floor() as i64),
        "by" => format!("{}", i.y.floor() as i64),
        "bz" => format!("{}", i.z.floor() as i64),
        "facing" => facing(i.yaw).into(),
        "yaw" => format!("{:.1}", i.yaw),
        "pitch" => format!("{:.1}", i.pitch),
        "dimension" => dimension_name(&i.dimension),
        _ => i.fps.to_string(),
    })
}

// ------------------------------------------------------------------ settings

thread_local! {
    /// The ON/OFF options, read once from their files, then kept.
    static FLAGS: RefCell<HashMap<&'static str, bool>> = RefCell::new(HashMap::new());
}

/// Whether option `key` (of TOGGLES) is ON, read once through `read` (the game's `native_setting`).
pub fn flag(key: &'static str, read: impl FnOnce(&str) -> Option<String>) -> bool {
    if let Some(v) = FLAGS.with(|f| f.borrow().get(key).copied()) {
        return v;
    }
    let v = read(key).is_none_or(|v| v.trim() != "false");
    FLAGS.with(|f| f.borrow_mut().insert(key, v));
    v
}

/// Sets option `key`; the caller keeps it with `native_set_setting`.
pub fn set_flag(key: &'static str, on: bool) {
    FLAGS.with(|f| f.borrow_mut().insert(key, on));
}

/// The settings page's button for an option: `Show the HUD box: ON`.
pub fn toggle_label(label: &str, on: bool) -> String {
    format!("{label}: {}", if on { "ON" } else { "OFF" })
}

/// The text kept for an option's file.
pub fn stored(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

// ------------------------------------------------------------------ keys and commands

/// What key `name` of KEYS does.
pub fn key_action(name: &str) -> Option<Action> {
    KEY_ACTIONS.iter().find(|(n, _)| *n == name).map(|(_, a)| *a)
}

/// What a chat command (without its slash, with its arguments) does, when it is one of this mod's.
pub fn command(command: &str) -> Option<Action> {
    let name = command.split_whitespace().next().unwrap_or("").to_lowercase();
    if name == ID {
        return Some(Action::Help);
    }
    COMMANDS.iter().find(|(n, _)| *n == name).map(|(_, a)| *a)
}

/// The /yepblocked lines: the mod, its commands and its keys. `key_label` gives the key a player chose.
pub fn help(key_label: impl Fn(&str, &str) -> String) -> Vec<String> {
    let mut out = vec![format!("{NAME} {VERSION}")];
    if !DESCRIPTION.is_empty() {
        out.push(DESCRIPTION.into());
    }
    if !COMMANDS.is_empty() {
        let names: Vec<String> = COMMANDS.iter().map(|(n, _)| format!("/{n}")).collect();
        out.push(format!("Commands: {}", names.join(", ")));
    }
    for [name, label, default] in KEYS {
        out.push(format!("{label}: {}", key_label(name, default)));
    }
    if !KEYS.is_empty() || !TOGGLES.is_empty() {
        out.push(format!("Change keys and options in Options > Mods > {NAME}."));
    }
    out
}

/// `key.keyboard.left.shift` as a player reads it: `Left Shift`.
pub fn key_label(key: &str) -> String {
    let short = key.trim_start_matches("key.keyboard.").trim_start_matches("key.");
    short
        .split('.')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ------------------------------------------------------------------ zoom

thread_local! {
    static ZOOMING: Cell<bool> = const { Cell::new(false) };
    static ZOOM_NOW: Cell<f32> = const { Cell::new(ZOOM_FACTOR) };
}

/// Zooms in or out (`Some(on)`), or the other way round (`None`). Zooming out puts the wheel's change back.
pub fn set_zoom(on: Option<bool>) {
    let on = on.unwrap_or(!ZOOMING.get());
    ZOOMING.set(on);
    if !on {
        ZOOM_NOW.set(ZOOM_FACTOR);
    }
}

/// How many times closer the view is now, or None while not zooming.
pub fn zoom_now() -> Option<f32> {
    ZOOMING.get().then(|| ZOOM_NOW.get())
}

/// The mouse's turning while zoomed: slower the closer the view, as with the spyglass.
pub fn zoom_turn(factor: f32) -> f32 {
    (1.0 / factor).clamp(0.05, 1.0)
}

/// Wheel steps while zooming change the zoom: whether they were taken (so the hotbar keeps still).
pub fn zoom_wheel(steps: f32) -> bool {
    if !ZOOM_SCROLL || !ZOOMING.get() {
        return false;
    }
    ZOOM_NOW.set((ZOOM_NOW.get() * 1.25f32.powf(steps)).clamp(1.5, 10.0));
    true
}

// ------------------------------------------------------------------ messages

thread_local! {
    static TICKS: Cell<u64> = const { Cell::new(0) };
    static IN_WORLD: Cell<bool> = const { Cell::new(false) };
}

/// One tick (20 a second) of the window's game: the lines to put in chat now. The welcome on joining a world,
/// then the timed messages every so many seconds, counted from joining.
pub fn tick(in_world: bool, welcome_on: bool, timers_on: bool) -> Vec<&'static str> {
    let mut say = Vec::new();
    let joined = in_world && !IN_WORLD.get();
    IN_WORLD.set(in_world);
    if !in_world {
        TICKS.set(0);
        return say;
    }
    if joined && welcome_on {
        say.extend_from_slice(WELCOME);
    }
    let t = TICKS.get() + 1;
    TICKS.set(t);
    if timers_on {
        for (every, lines) in TIMERS {
            if t.is_multiple_of(u64::from(*every) * 20) {
                say.extend_from_slice(lines);
            }
        }
    }
    say
}

#[cfg(test)]
mod tests {
    use super::*;

    fn here() -> Info {
        Info::new((12.34, 64.0, -7.5, 180.0, -10.0, "minecraft:the_nether"), 144)
    }

    #[test]
    fn placeholders_fill_in() {
        let i = here();
        assert_eq!(fill("XYZ {x} {y} {z}", Some(&i)), "XYZ 12.3 64.0 -7.5");
        assert_eq!(fill("{bx} {by} {bz}", Some(&i)), "12 64 -8");
        assert_eq!(fill("{facing} in the {dimension}", Some(&i)), "North (-Z) in the Nether");
        assert_eq!(fill("{fps} fps", Some(&i)), "144 fps");
        assert_eq!(fill("{nope} {x", Some(&i)), "{nope} {x");
        assert_eq!(fill("{x} {name}", None), format!("? {NAME}"));
    }

    #[test]
    fn facing_follows_minecraft() {
        assert_eq!(facing(0.0), "South (+Z)");
        assert_eq!(facing(90.0), "West (-X)");
        assert_eq!(facing(-90.0), "East (+X)");
        assert_eq!(facing(179.0), "North (-Z)");
    }

    #[test]
    fn the_help_command_and_commands_are_found() {
        assert_eq!(command("yepblocked"), Some(Action::Help));
        assert_eq!(command("yepblocked extra words"), Some(Action::Help));
        for (name, action) in COMMANDS {
            assert_eq!(command(&name.to_uppercase()), Some(*action));
        }
        assert_eq!(command("not_a_command_of_this_mod"), None);
        assert!(help(|_, d| key_label(d)).len() >= 1);
    }

    #[test]
    fn every_key_has_an_action() {
        for [name, _, default] in KEYS {
            assert!(key_action(name).is_some(), "{name}");
            assert!(default.starts_with("key."), "{default}");
        }
        assert_eq!(key_label("key.keyboard.left.shift"), "Left Shift");
    }

    #[test]
    fn options_start_on_and_keep_their_value() {
        for &(key, _) in TOGGLES {
            assert!(flag(key, |_| None));
            set_flag(key, false);
            assert!(!flag(key, |_| Some("true".into())));
            set_flag(key, true);
        }
        assert!(!flag("stored_off", |_| Some("false".into())));
    }

    #[test]
    fn zoom_goes_in_and_out() {
        assert_eq!(zoom_now(), None);
        set_zoom(Some(true));
        assert_eq!(zoom_now(), Some(ZOOM_FACTOR));
        assert_eq!(zoom_wheel(1.0), ZOOM_SCROLL);
        set_zoom(None);
        assert_eq!(zoom_now(), None);
        assert!(zoom_turn(4.0) < 1.0);
    }

    #[test]
    fn welcome_once_per_join_and_timers_on_time() {
        assert!(tick(false, true, true).is_empty());
        assert_eq!(tick(true, true, false), WELCOME.to_vec());
        assert!(tick(true, true, false).is_empty());
        tick(false, true, true);
        let mut said = 0;
        for _ in 0..20 * 60 * 60 {
            said += tick(true, false, true).len();
        }
        let want: usize = TIMERS.iter().map(|(e, l)| (3600 / *e as usize) * l.len()).sum();
        assert_eq!(said, want);
    }
}
