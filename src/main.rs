use pollster;
use wgpu_learn::run;

fn main() {
    pollster::block_on(run());
    println!("start");
}
