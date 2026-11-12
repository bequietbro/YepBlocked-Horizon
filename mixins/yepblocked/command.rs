// The mod's chat commands (COMMANDS in runtime.rs) and /yepblocked, which lists them. Horizon asks before the
// server sees a command: `ran` keeps it from going on.
fn yepblocked_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::kit_yepblocked as m;
    // Turned off in Options > Mods: nothing until the launcher's next build leaves the mod out.
    if !game.native_on(m::ID) {
        return;
    }
    let Some(action) = m::command(command) else {
        return;
    };
    *ran = true;
    match action {
        m::Action::Say(lines) => {
            let info = game.native_player().map(|p| m::Info::new((p.x, p.y, p.z, p.yaw, p.pitch, p.dimension), game.native_frame_rate().0));
            for line in lines {
                game.native_chat(&m::fill(line, info.as_ref()));
            }
        }
        m::Action::ToggleHud => {
            let on = !m::flag("hud", |k| game.native_setting(m::ID, k));
            m::set_flag("hud", on);
            game.native_set_setting(m::ID, "hud", m::stored(on));
        }
        m::Action::ToggleZoom | m::Action::HoldZoom => {
            if m::flag("zoom", |k| game.native_setting(m::ID, k)) {
                m::set_zoom(None);
            }
        }
        m::Action::Help => {
            let lines = m::help(|name, default| m::key_label(&game.native_mod_key(m::ID, name, default)));
            for line in lines {
                game.native_chat(&line);
            }
        }
    }
}
