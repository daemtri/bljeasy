pub mod cmd;

fn main() {
    cmd::execute().expect("服务运行失败");
}
