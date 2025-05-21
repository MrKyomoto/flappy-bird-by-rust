use std::str::EncodeUtf16;

use bracket_lib::prelude::*;

enum GameMode{
    Menu,
    Playing,
    End,
}
// State用于保存每一帧的状态
struct State{
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm){
        ctx.print_centered(10,"test");
        self.mode = GameMode::End;
    }

    fn restart(&mut self){
        self.mode = GameMode::Playing;
    }
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls(); 
        ctx.print_centered(5, "Welcome to Flappy Bird");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls(); 
        ctx.print_centered(5, "Game End");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
        
    }
}

impl GameState for State{
    // ctx = context 上下文, 此处相当于游戏窗口
   fn tick(&mut self, ctx: &mut BTerm){
    match self.mode {
        GameMode::Menu => self.main_menu(ctx), 
        GameMode::End => self.dead(ctx),
        GameMode::Playing => self.play(ctx),
    }
   } 
}
fn main() -> BError {
    //游戏终端实例
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Bird")
        .build()?; // 文末的 ? 表示构建可能出错,如果出错就返回BErr
    main_loop(context, State::new())
}

