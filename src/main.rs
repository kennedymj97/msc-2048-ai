use msc_2048_ai::ai;
use msc_2048_ai::ui;

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(3));
    ai::run_ai();
    //ui::start_game_in_ui();
    std::thread::sleep(std::time::Duration::from_secs(3));
}
