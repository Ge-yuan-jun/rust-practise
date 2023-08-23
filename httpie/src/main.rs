use anyhow::{{anyhow, Result}};
use reqwest::{{header, Client, Response, Url}};
use clap::Parser;
use std::{collections::HashMap, str::FromStr};

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
    // http 请求的 url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post function
#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>
}

// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String
}

// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("fail to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}