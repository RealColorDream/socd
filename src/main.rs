use std::time::Duration;
use enigo::{Direction::{Press, Release}, Enigo, Key, Keyboard, Settings};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let device_state = DeviceState::new();
    let mut last_key: Option<Keycode> = None;

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        let z_pressed = keys.contains(&Keycode::Z);
        let s_pressed = keys.contains(&Keycode::S);
        let q_pressed = keys.contains(&Keycode::Q);
        let d_pressed = keys.contains(&Keycode::D);

        // Handle vertical SOCD (Z and S)
        if z_pressed && s_pressed {
            // Neutralize both
            if let Some(last) = last_key {
                let _ = enigo.key(match last {
                    Keycode::Z => Key::Z,
                    Keycode::S => Key::S,
                    _ => Key::Space, // Default case to avoid unreachable
                }, Release);
            }
            last_key = None;
        } else if z_pressed {
            if last_key != Some(Keycode::Z) {
                if let Some(last) = last_key {
                    let _ = enigo.key(match last {
                        Keycode::Z => Key::Z,
                        Keycode::S => Key::S,
                        _ => Key::Space, // Default case to avoid unreachable
                    }, Release);
                }
                let _ = enigo.key(Key::Z, Press);
                last_key = Some(Keycode::Z);
            }
        } else if s_pressed {
            if last_key != Some(Keycode::S) {
                if let Some(last) = last_key {
                    let _ = enigo.key(match last {
                        Keycode::Z => Key::Z,
                        Keycode::S => Key::S,
                        _ => Key::Space, // Default case to avoid unreachable
                    }, Release);
                }
                let _ = enigo.key(Key::S, Press);
                last_key = Some(Keycode::S);
            }
        }

        // Handle horizontal SOCD (Q and D)
        if q_pressed && d_pressed {
            // Neutralize both
            if let Some(last) = last_key {
                let _ = enigo.key(match last {
                    Keycode::Q => Key::Q,
                    Keycode::D => Key::D,
                    _ => Key::Space, // Default case to avoid unreachable
                }, Release);
            }
            last_key = None;
        } else if q_pressed {
            if last_key != Some(Keycode::Q) {
                if let Some(last) = last_key {
                    let _ = enigo.key(match last {
                        Keycode::Q => Key::Q,
                        Keycode::D => Key::D,
                        _ => Key::Space, // Default case to avoid unreachable
                    }, Release);
                }
                let _ = enigo.key(Key::Q, Press);
                last_key = Some(Keycode::Q);
            }
        } else if d_pressed {
            if last_key != Some(Keycode::D) {
                if let Some(last) = last_key {
                    let _ = enigo.key(match last {
                        Keycode::Q => Key::Q,
                        Keycode::D => Key::D,
                        _ => Key::Space, // Default case to avoid unreachable
                    }, Release);
                }
                let _ = enigo.key(Key::D, Press);
                last_key = Some(Keycode::D);
            }
        }

         std::thread::sleep(Duration::from_millis(90));
    }
}