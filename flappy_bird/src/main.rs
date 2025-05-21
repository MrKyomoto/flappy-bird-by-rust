use bracket_lib::prelude::*;

// State用于保存每一帧的状态
struct State{}

impl GameState for State{
    // ctx = context 上下文, 此处相当于游戏窗口
   fn tick(&mut self, ctx: &mut BTerm){
    ctx.cls();//清理屏幕
    ctx.print(1 ,1 ,"Hello, Bracket Terminal!");//(x,y)为屏幕坐标,左上角为(0,0), 打印一句话到屏幕中
   } 
}
fn main() -> BError {
    //游戏终端实例
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Bird")
        .build()?; // 文末的 ? 表示构建可能出错,如果出错就返回BErr

    main_loop(context, State{})
}

