cfg_select! {
    feature = "terminal_window" => {
        pub mod terminal;
        pub use terminal::Window as Window;
    }
    feature = "rata" => {
        pub mod ratatui;
        pub use ratatui::Window as Window;
    }
    feature = "sdl3_window" => {
        pub mod sdl3;
        pub use sdl3::Window as Window;
    }
}
