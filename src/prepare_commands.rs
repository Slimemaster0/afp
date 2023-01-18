use crate::Exec;
use crate::items::*;
use crate::common_functions::nop;

pub async fn prepare_commands(items: &Vec<Item>) -> Vec<String> {
    let mut output_vec: Vec<String> = vec![];
    for item in items.iter() {
        match item {

        Item::Command(current_item) => {
            let command = Exec { cmd: current_item.command.to_owned(), args: current_item.args.to_owned() };
            let output = command.get_output();
            output_vec.push(output);
        },

        Item::LineCount(current_item) => {
            let command = Exec { cmd: current_item.command.to_owned(), args: current_item.args.to_owned() };
            let output = command.get_output();
            let current_item_output: Vec<&str> = output.split("\n").collect();
            output_vec.push(current_item_output.len().to_string())
        },
        
        _ => nop()
        };
    };
    return output_vec;
}
