use std::{fmt::format, str::EncodeUtf16};

use bracket_lib::prelude::*;

enum GameMode{
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 32.5; //每32.5ms做一些事情
const MIN_OBSTACLE_DISTANCE: i32 = 20; // 障碍之间的最小距离
const MAX_OBSTACLE_DISTANCE: i32 = 30; // 障碍之间的最大距离
const MAX_OBSTACLES: i32 = 5; // 最多同时存在的障碍数量

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
        ctx.set(1,self.y,YELLOW,BLACK,to_cp437('@'));
    }

    fn gravity_and_move(&mut self){
        //MEMO(注意坐标轴是往下的,即左上角0,0,往下为y的正方向)
        //往下的最大速度是2.0
        if self.v < 2.0 {
            self.v += 0.1;
        }

        self.y += self.v as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self){
        self.v = -2.0;
    }

    fn dash(&mut self, key: VirtualKeyCode){
        match key {
            VirtualKeyCode::A => self.x += -6, 
            VirtualKeyCode::D => self.x += 6, 
            _ => {}
        }
    }
}

// State用于保存每一帧的状态
struct State{
    mode: GameMode,
    player: Player,
    frame_time: f32, //游戏累计了多少帧以后,共多少时间
    obstacles: ObstacleManager,
    score: i32,
}

impl State {
    fn new() -> Self {
        let mut obstacles = ObstacleManager::new();
        obstacles.generate_obstacles(SCREEN_WIDTH, 0);
        
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25, 0.0),
            frame_time: 0.0,
            obstacles,
            score: 0,
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
        ctx.print_centered(1,&format!("Score: {}",self.score));

        // 渲染所有障碍
        self.obstacles.render(ctx, self.player.x);

        // 检查是否通过障碍并更新分数
        if self.obstacles.check_passed(&self.player) {
            self.score += 1;
        }

        // 生成新的障碍
        self.obstacles.update(self.player.x, self.score);

        // 检查碰撞
        if self.player.y > SCREEN_HEIGHT || self.obstacles.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self){
        self.player = Player::new(5, 25, 0.0);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacles.reset();
        self.obstacles.generate_obstacles(SCREEN_WIDTH, 0);
        self.score = 0;
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
        ctx.print_centered(6, &format!("You earned {} points",self.score));
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

struct Obstacle{
    x: i32, // x是整个游戏空间的横坐标
    gap_y: i32,
    size: i32,
    passed: bool, // 是否已被玩家通过
    // gap_y - (size / 2) to gap_y + (size / 2)
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self{
        let mut random = RandomNumberGenerator::new();
        Obstacle { 
            x, 
            gap_y: random.range(10, 40), 
            size: i32::max(2, 20 - (score / 5)), // 随着分数增加，间隙变小
            passed: false,
        }
    } 

    fn render(&self, ctx: &mut BTerm, player_x: i32){
        let screen_x = self.x - player_x; //这个是屏幕空间的坐标
        let half_size = self.size / 2;
        
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let is_in_x_range = player.x >= self.x - 1 && player.x <= self.x + 1; // 稍微放宽x轴判定范围
        let is_above_y =  player.y < self.gap_y - half_size;
        let is_below_y = player.y > self.gap_y + half_size;
        
        is_in_x_range && (is_above_y || is_below_y)
    }
}

// 障碍管理器，负责生成和管理多个障碍
struct ObstacleManager {
    obstacles: Vec<Obstacle>,
}

impl ObstacleManager {
    fn new() -> Self {
        ObstacleManager {
            obstacles: Vec::new(),
        }
    }
    
    fn reset(&mut self) {
        self.obstacles.clear();
    }
    
    // 生成一组随机数量的障碍
    fn generate_obstacles(&mut self, start_x: i32, score: i32) {
        let mut random = RandomNumberGenerator::new();
        let count = random.range(2, MAX_OBSTACLES + 1); // 随机生成2到MAX_OBSTACLES个障碍
        
        let mut current_x = start_x;
        
        for _ in 0..count {
            // 确保障碍之间有足够的距离
            let distance = random.range(MIN_OBSTACLE_DISTANCE, MAX_OBSTACLE_DISTANCE);
            current_x += distance;
            
            self.obstacles.push(Obstacle::new(current_x, score));
        }
    }
    
    // 渲染所有障碍
    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        for obstacle in &self.obstacles {
            // 只渲染屏幕内的障碍
            let screen_x = obstacle.x - player_x;
            if screen_x > -5 && screen_x < SCREEN_WIDTH {
                obstacle.render(ctx, player_x);
            }
        }
    }
    
    // 检查是否与任何障碍碰撞
    fn hit_obstacle(&self, player: &Player) -> bool {
        for obstacle in &self.obstacles {
            if obstacle.hit_obstacle(player) {
                return true;
            }
        }
        false
    }
    
    // 检查是否通过了任何障碍
    fn check_passed(&mut self, player: &Player) -> bool {
        let mut passed = false;
        
        for obstacle in &mut self.obstacles {
            if !obstacle.passed && player.x > obstacle.x {
                obstacle.passed = true;
                passed = true;
            }
        }
        
        passed
    }
    
    // 更新障碍状态，必要时生成新障碍
    fn update(&mut self, player_x: i32, score: i32) {
        // 移除已经在玩家后方很远的障碍
        self.obstacles.retain(|o| o.x > player_x - SCREEN_WIDTH);
        
        // 如果没有足够的障碍，生成新的
        if self.obstacles.is_empty() || self.obstacles.last().unwrap().x < player_x + SCREEN_WIDTH * 2 {
            let last_x = if let Some(last) = self.obstacles.last() {
                last.x
            } else {
                player_x + SCREEN_WIDTH
            };
            
            self.generate_obstacles(last_x, score);
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