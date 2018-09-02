extern crate cyjproject;
use std::env;
use std::process;

use cyjproject::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("problem parsing arguments:{}",err);
        process::exit(1);
    });

    println!("search for : {}", config.query);
    println!("file : {}", config.filename);

    if let Err(e)=cyjproject::run(config){
        println!("Error:{}",e);
        process::exit(1);
    };
}

//main中包含 使用参数值调用命令行解析逻辑
//          设置配置
//          调用run函数
//          处理run返回的err




    /*
    let query=&args[1];
    let filename=&args[2];
    println!("search for : {}",query);
    println!("file : {}",filename);
    //传递filename来获取文件 expect内是失败时打印出的错误信息
    let mut f = File::open(filename).expect("file not found");
    //可变的contents用于存放文件的内容
    let mut contents=String::new();
    //read_to_string传递参数
    f.read_to_string(&mut contents).expect("wrong with reading the file");
    println!("text: \n{}",contents);
}*/