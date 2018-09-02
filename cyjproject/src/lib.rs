use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
//lib中内容  run函数的定义
//          相关的use语句
//          config


pub struct Config{
    pub query:String,
    pub filename:String,
   // pub judgesensi:bool,
}
impl Config{
    pub fn new(args:&[String])->Result<Config,&'static str> {
        if args.len()<3{
            return Err("not enoufh arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok( Config{ query, filename })
    }
}
pub fn run(config:Config) ->Result<(),Box<Error>>{
    let mut f =File::open(config.filename)?;
    let mut contents =String::new();
    f.read_to_string(&mut contents)?;
    for line in search(&config.query,&contents){
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut resline=Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            resline.push(line);
        }
    }
    resline
}

pub fn insensitivesearch<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let query =query.to_lowercase();
    let mut resline=Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            resline.push(line);
        }
    }
    resline
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn sensitivecase(){
        let query="cyj";
        let contents="\
Come on,
cyj,
you are the best.";

        assert_eq!(vec!["cyj,"],search(query,contents));
    }

    #[test]
    fn insesitivecase(){
        let query="CYJ";
        let contents="\
Come on,
cyj,
you are the best.";

        assert_eq!(vec!["cyj,"],insensitivesearch(query,contents));
    }
}
