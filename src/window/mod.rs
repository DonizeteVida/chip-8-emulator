cfg_select! {
    feature = "term" => {
        pub mod terminal;
        pub use terminal::Window as Window;
    }
    feature = "rata" => {
        pub mod ratatui;
        pub use ratatui::Window as Window;
    }
}
