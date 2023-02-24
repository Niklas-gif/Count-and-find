use std::io;

struct Word{
   pub word: String,
   pub count: i32
}

impl Word {
    fn new(word: String, count: i32) -> Self { Self { word, count } }
}

fn main() {
    println!("Enter the text to display word count or type quite: to quite");
    //let text : String = String::from("Hello");
    let mut word = Word{
        word: String::from("Chicken"),
        count:0
    };
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    if !input.contains("quite:") {
        parse(&input,&mut word);
        println!("Word:{} amount:{}",word.word,word.count);
        println!("Text:{}",input);
    }
    else{
        println!("Bye Bye!");
        input.clear();
    }
}

fn parse(text: &String,word : &mut Word){
    let text_low = text.to_lowercase().clone();
    for i in text_low.split(" "){
        if i.contains(&word.word.to_lowercase()){
            word.count = word.count + 1;
        }
    } 
}
