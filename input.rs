use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::*;

const REGS: [&str; 4] = ["A", "B", "C", "D"];

fn return_type(token: &str, label_to_address: &HashMap<String, usize>) -> Option<&'static str>{
    let mut token = token.to_string();
    let mut is_address = false;
    if token.starts_with('[') && token.ends_with(']'){
        is_address = true;
        token = token.replace("[","");
        token = token.replace("]","");
    }

    // if token represents int
    if let Ok(_i) = token.parse::<u32>(){
        if is_address{
            return Some("[const]")
        }else{
            return Some("const")
        }
    };

    // if token represents reg
    if REGS.contains(&&token[..]){
        if is_address{
            return Some("[reg]")
        }else{
            return Some("reg")
        }
    }

    // if token represents a label
    if label_to_address.contains_key(&token){
        if is_address{
            return Some("[const]")
        }else{
            return Some("const")
        }
    }
    None
}

fn tokens_to_instruc(tokens: &[String], label_to_address: &HashMap<String, usize>) -> Option<String>{
    let mut instruc = "".to_string();

    if tokens.is_empty(){
        return None;
    }

    instruc = instruc + &tokens[0];

    if tokens.len() > 1{
        instruc = instruc + "_" + return_type(&tokens[1], label_to_address)?;
    }
    if tokens.len() > 2{
        instruc = instruc + "_" + return_type(&tokens[2], label_to_address)?;
    }

    Some(instruc)
}

pub fn assemble(assembly_code: &str) -> Vec<u8>{

    // get mapping for instructions to binary        
        let instruc_to_binary_file = include_str!("../instrucToBinary.json");
        let instruc_to_binary: serde_json::Value = serde_json::from_str(&instruc_to_binary_file).expect("JSON was not well-formatted");
    
    // parse assembly_code into tokens
        let lines: Vec<&str> = assembly_code.split('\n').collect();

        // remove comments (anything after a ";" char)
            let mut lines_no_comments = Vec::new();
            for line in lines.into_iter() {
                let commentless_lines = match line.find(';') {
                    Some(x) => line[0..x].to_string(),
                    None => line.to_string()
                };

                lines_no_comments.push(commentless_lines);
            }
        // remove ","
            lines_no_comments = lines_no_comments.into_iter().map(|x| x.replace(",", "")).collect();
        // remove "\r"
            lines_no_comments = lines_no_comments.into_iter().map(|x| x.replace("\r", "")).collect();

        // tokenize 
            // tokenizes but yeilds some empty string tokens
                let tokens: Vec<Vec<String>> = lines_no_comments.into_iter().map(|x| x.split(' ').map(|x| x.to_string()).collect()).collect();
            // remove empty string tokens
                let emptyless_tokens: Vec<Vec<String>> = tokens.into_iter().map(|x| x.into_iter().filter(|x| !x.is_empty()).collect()).collect();
            // remove empty lines
                let raw_tokens: Vec<Vec<String>> = emptyless_tokens.into_iter().filter(|x| !x.is_empty()).collect();

        // find all labels and their addresses
            let mut address = 0;
            let mut label_to_address = HashMap::new();

            for tokens in &raw_tokens{
                if tokens.len() == 1 && tokens[0].ends_with(':'){
                    label_to_address.insert(
                        tokens[0][0..tokens[0].len()-1].to_string(),
                        address
                    );
                }else{
                    address += tokens.len()
                }
            }

        // remove lable definitions from tokens
            let raw_tokens: Vec<Vec<String>> = raw_tokens.into_iter().filter(|x| !x[0].ends_with(':')).collect();

    // turn tokens into machine code
        let mut machine_code: Vec<u8> = Vec::new();
        

        for tokens in raw_tokens{
            // find machine code for the instruction
                let instruc_string = tokens_to_instruc(&tokens, &label_to_address).expect("tokens unable to form instruction");
                let instruction_machine_code = instruc_to_binary.get(&instruc_string).expect("no machine code for requested instruction").as_u64().expect("ISA instruction mapped to non integer value");
                machine_code.push(instruction_machine_code as u8);
            // append machine code of any operands
                for (i,token) in tokens.iter().enumerate(){
                    // skip the first instruction which was already encoded
                    if i != 0{
                        // if reg, append reg value
                            if REGS.contains(&&token[..]){
                                machine_code.push(REGS.iter().position(|x| x == &&token[..]).expect("REG not in REGS") as u8);
                            }
                        // if const, append const value
                            else if let Ok(i) = token.parse::<i32>(){
                                machine_code.push(i as u8);
                            }

                        // if label, append label value
                            else if label_to_address.contains_key(token){
                                machine_code.push(*label_to_address.get(token).unwrap() as u8)
                            }
                        // throw error if invalid token
                            else{
                                panic!("operand token cannot be converted to machine code");
                            }
                    }
                }
        }
    machine_code
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assemble(){
        let machine_code = super::assemble("
            counting:
            MOV A, 0
            .countLoop:
            PRINT A
            INC A
            CMP A, 11
            JNC .countLoop
        ");

        let raw_bytes: Vec<u8> = [
            3,0,0,
            25,0,
            8,0,
            27,0,11,
            14,3
        ].to_vec();
    
        assert_eq!(machine_code, raw_bytes)
    }
}