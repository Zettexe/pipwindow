use hyprland::{
    data::Clients,
    dispatch::{Dispatch, DispatchType, WindowIdentifier, Position},
    event_listener::EventListener,
    shared::HyprData,
};

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    println!("Started watching for windows");


    event_listener.add_window_title_change_handler(|address| {
        if let Some(client) = Clients::get()
            .unwrap()
            .find(|client| client.address.to_string() == format!("0x{address}"))
        {
            if client.title == "Picture-in-Picture" && !client.floating {
                let window_dim_x: f32 = 1920.0;
                let window_dim_y: f32 = 1080.0;
                Dispatch::call(DispatchType::ToggleFloating(Some(WindowIdentifier::Title(
                    "Picture-in-Picture",
                ))))
                .unwrap();
                println!("Floated window");
                Dispatch::call(DispatchType::ResizeActive(Position::Exact((window_dim_x * 0.3) as i16, (window_dim_y * 0.3) as i16)))
                .unwrap();
                println!("Resized window");
                Dispatch::call(DispatchType::MoveActive(Position::Exact((window_dim_x - window_dim_x * 0.3) as i16 - 1, (window_dim_y - window_dim_y * 0.3) as i16 - 1)))
                .unwrap();
                println!("Moved window");
            }
        }
    });

    event_listener.start_listener()
}
