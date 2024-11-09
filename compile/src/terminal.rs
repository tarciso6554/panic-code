use clap::builder::Str;
use nom::InputLength;

use std::io;
pub fn terminal_main(){
   
      while(true){
        let  prompt = "exit";
        let input_len = prompt.input_len();
        if(input_len.to_string()=="exit"){
           print!("ola mundo")

            } else{
                break;
            }
      }
}