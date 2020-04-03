pub mod basic;
pub mod initial;

pub trait GameEngine {
    type Board;

    fn new() -> Self;

    fn get_state(&mut self) -> &mut Self::Board;

    fn move_left(&mut self);

    fn move_right(&mut self);

    fn move_up(&mut self);

    fn move_down(&mut self);
}
