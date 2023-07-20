use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "geyuanjun daydaygyj@gmail.com")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get function
#[derive(Parser, Debug)]
struct Get {
    url: String,
}

// post function
#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}