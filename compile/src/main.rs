mod terminal;

use std::io;
use clap::parser;
use rayon;

fn main() {
   terminal::terminal_main();
 
 }

struct  processo_compilacao{
    codigo_fonte: String,
    avore_sitatica:Option<Avore_sitatica>,
    sibolos:Option<Sibolos>,


}

struct Avore_sitatica;
struct Sibolos;

impl processo_compilacao{
    pub fn parse(_: &mut Self){

    }fn sematica(){

    }
}impl Avore_sitatica{
    pub fn new()->Avore_sitatica{
        Avore_sitatica{}
    }
}impl Sibolos{
    fn new()->Self{
        Sibolos{}
    }

    }