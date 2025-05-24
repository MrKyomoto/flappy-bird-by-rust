use std::str::EncodeUtf16;

use bracket_lib::prelude::*;

enum GameMode{
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 32.5; //每32.5ms做一些事情
struct Player{
    x:i32,
    y:i32,
    v:f32, //velocity - 垂直速度, if less than zero then fall down, vise versa; 为浮点类型是为了让玩家下落的过程更丝滑
}

impl Player {
    fn new(x:i32,y:i32,v:f32) -> Self{
        Player { 
            x: x,
            y: y, 
            v: v, 
        }
    }

    fn render(&mut self, ctx: &mut BTerm){
        // ctx.set(self.x,self.y,YELLOW,BLACK,to_cp437('@'));
        ctx.set(1,self.y,YELLOW,BLACK,to_cp437('@'));
    }

    fn gravity_and_move(&mut self){
        //MEMO(注意坐标轴是往下的,即左上角0,0,往下为y的正方向)
        //往下的最大速度是2.0
        if self.v < 2.0 {
            self.v += 0.1;
        }

        self.y += self.v as i32;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self){
        //每按一次空格,就会让速度变为向上的2.0
        // if self.v > -2.0 {
        //     self.v += -2.0;
        // }
        self.v = -2.0;
    }

    fn dash(&mut self, key: VirtualKeyCode){
        match key {
            VirtualKeyCode::A => self.x += -2, 
            VirtualKeyCode::D => self.x += 2, 
            _ => {}
        }
    }
}

// State用于保存每一帧的状态
struct State{
    mode: GameMode,
    player:Player,
    frame_time:f32, //游戏累计了多少帧以后,共多少时间
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25, 0.0),
            frame_time: 0.0
        }
    }

    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        //不要放在if frame_time的逻辑里, 为了保证随时按空格都有反应
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::A => self.player.dash(key),
                VirtualKeyCode::D => self.player.dash(key),
                _ => {},
            }
        }

        self.player.render(ctx);
        ctx.print_centered(SCREEN_HEIGHT - 2,"Press Space to Flap");

        if self.player.y > SCREEN_HEIGHT{
            self.mode = GameMode::End;
        }
        
    }

    fn restart(&mut self){
        self.player = Player::new(5, 25, 0.0);
        self.frame_time = 0.0;
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

