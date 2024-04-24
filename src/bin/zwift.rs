use std::collections::BTreeMap;
use std::path::Path;
use std::{thread, time};
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        hide_self();
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.name == "zproject_selection" {
            match pipe_message.payload {
                Some(payload) => {
                    let cwd = Path::new(payload.trim());
                    let name = cwd.file_stem().unwrap().to_str().unwrap();
                    eprintln!("Selected project: {name}, cwd {payload}");
                    close_focus();
                    eprintln!("sleeping...");
                    thread::sleep(time::Duration::from_secs(5));
                    eprintln!("switching...");
                    switch_session_with_layout(
                        Some(name),
                        LayoutInfo::File("dev".into()),
                        Some(cwd.to_path_buf()),
                    );
                }
                _ => {
                    eprintln!("No payload in message");
                }
            }
        }
        false
    }
}