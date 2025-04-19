use wgpu_learn::run;
use pollster;

fn main() {
    pollster::block_on(run());
    println!("start");

}
