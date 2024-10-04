pub fn functions(input: String) -> Vec<String> {
    let mut output = Vec::new();
    let mut temp = String::new();
    
    for i in input.chars() {
        if i == ';'
        || i == '\n'{
            if !temp.is_empty() {
                output.push(temp.clone().trim().to_string());
                temp.clear();
            }
        } else {
            temp.push(i);
        }
    }
    output.push(temp.trim().to_string());
    
    return output;
}

pub fn components(input: String) -> Vec<String> {
    let mut output = Vec::new();
    let mut temp = String::new();
    
    let mut count = 0; // Consecutive spaces counter.
    let mut numeric = false; // Detects whether a character is numeric.
    
    for i in input.chars() {
        match i {
            ' ' => {
                if !temp.is_empty(){
                    output.push(temp.clone());
                    temp.clear();
                }
                count += 1; // "Store" spaces.
            }
            '+'|'-'|'*'|'/'
           |'^'|'('|')'|'!'
           |'~'|'\\' => { // Signals.
               if !temp.is_empty(){
                   output.push(temp.clone());
                   temp.clear();
               }
               
               output.push(i.to_string());
            }
            _ => {
                if count > 0 {
                    // Return stored spaces.
                    for _ in 0..count {
                        temp.push(' ');
                    }
                    count = 0;
                    output.push(temp.clone());
                    temp.clear();
                }
                // separates numbers from other characters.
                let prev_numeric = numeric;
                if i.is_numeric() || i == '.'{
                    numeric = true;
                } else {
                    numeric = false;
                }
                
                if prev_numeric != numeric {
                    if !temp.is_empty() {
                        output.push(temp.clone());
                        temp.clear();
                    }
                }
                
                temp.push(i)
            }
        }
    }
    output.push(temp);
    
    // Fix some problems.
    
    // Separating numbers sometimes
    // leaves the first item empty.
    if output[0].is_empty(){
        // Delete it.
        output.remove(0);
    }
    
    let mut index = 0;
    while index < output.len() {
        match output[index].as_str() {
            // Forms multi-character commands
            // using the "!" and "~" commands.
            "!" => {
                // '!' stands for actual functions.
                // Example: rphnew! abc
                if !output[index-1].chars().all(char::is_whitespace){
                    output[index-1].push('!');
                    output[index].clear();
                }
            }
            "~" => {
                // '~' represents paramters.
                // Example: rphnew! abc ~public
                if !output[index+1].chars().all(char::is_whitespace){
                    output[index+1].insert(0, '~');
                    output[index].clear();
                }
            }
            _ => {
                // Join "-" signal when it detects
                // that it is meant for a negative
                // number.
                
                // Removes repetitive signals.
                
            }
        }
        
        index += 1;
    }
    
    return output;
}