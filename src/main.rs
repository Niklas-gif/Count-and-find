use std::io;

#[derive(Eq)]
#[derive(Debug)]
struct Word{
   pub word: String,
   pub count: i32
}
impl PartialEq for Word{
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

impl Word {
    fn new(word: String, count: i32) -> Self { Self { word, count } }
}

fn main() {
    println!("Enter the text to display word count or type quite: to quite");
    let mut input = String::new();
    let mut words:Vec<Word> = Vec::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if input.ends_with('\n') {
         input = input[0..input.len() - 1].to_string();
        }
    
    if !input.contains("quite:") {
        parse(&input,&mut words);
        print_words(&words);
    }
    else{
        println!("Bye Bye!");
        input.clear();
    }
}

fn parse(text: &String,words:&mut Vec<Word>){
    let text_low = text.to_lowercase().clone();
    for text_word in text_low.split(" "){
        if !(words.iter().any(|word| word.word.to_lowercase().eq(text_word))){
            words.push(Word::new(String::from(text_word),0));
        }
        for i in 0..words.len(){
        if words[i].word.to_lowercase().eq(text_word){
            words[i].count = words[i].count + 1;
        }
    }
   }
}

fn print_words(words:&Vec <Word>){
    for word in words{
        println!("Word: {} amount: {} \n------------------",word.word,word.count);
    }
}