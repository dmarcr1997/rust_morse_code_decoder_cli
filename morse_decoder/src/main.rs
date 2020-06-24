use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>
}

impl MorseDecoder {
    fn decode_morse(&self, encoded: &str) -> String {
        let refined: Vec<String> = encoded
                .split(" ")
                .map(|s| s.parse().expect("parse error"))
                .collect();
        println!("{:?}", refined);
        let mut decoded_string: Vec<String> = vec![];
        let mut space_count = 0;
        for code in refined{
            if code == ""{
                if space_count < 1{
                    space_count += 1;
                    decoded_string.push(" ".to_string());
                }
                else{
                    space_count = 0;
                }
            }
            else{
                for (k, v) in self.morse_code.iter(){
                    if k.to_string() == code{
                        decoded_string.push(v.to_string());
                        break;
                    }
                }
            }
        }
        return decoded_string.join("").trim().to_string() 
    }
    
}

fn main() {
    let morse_key = vec![
       ".-".to_string(), 
       "-...".to_string(), 
       "-.-.".to_string(), 
       "-..".to_string(), 
       ".".to_string(), 
       "..-.".to_string(), 
       "--.".to_string(), 
       "....".to_string(), 
       "..".to_string(), 
       ".---".to_string(), 
       "-.-".to_string(), 
       ".-..".to_string(), 
       "--".to_string(), 
       "-.".to_string(), 
       "---".to_string(), 
       ".--.".to_string(), 
       "--.-".to_string(), 
       ".-.".to_string(), 
       "...".to_string(), 
       "-".to_string(), 
       "..-".to_string(), 
       "...-".to_string(), 
       ".--".to_string(), 
       "-..-".to_string(), 
       "-.--".to_string(), 
       "--..".to_string()  
    ];

    let letters = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
        "K".to_string(),
        "L".to_string(),
        "M".to_string(),
        "N".to_string(),
        "O".to_string(),
        "P".to_string(),
        "Q".to_string(),
        "R".to_string(),
        "S".to_string(),
        "T".to_string(),
        "U".to_string(),
        "V".to_string(),
        "W".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string() 
    ];

    let decoder = MorseDecoder {
        morse_code: morse_key.into_iter().zip(letters.into_iter()).collect()
    };
     
    println!("{}", decoder.decode_morse("-.. .- -- --- -."));
}


