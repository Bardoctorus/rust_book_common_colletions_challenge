use rand::prelude::*;
use std::collections::HashMap;
use std::env;
use std::process;

// TODO: Return values.
fn int_med_mode(){
    // Put random numbers in a Vec.
    println!("-----------------------------");
    println!("Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.");
    println!("-----------------------------");
    let mut rng = thread_rng();
    let max_num = rng.gen_range(5..5000);
    println!("Generating a vector whose values and length can be any integers up to {}", max_num);
    let mut in_vec : Vec<i32> = Vec::new();
    for _i in 0..max_num{
    let x :i32 = rng.gen_range(0..max_num);
    in_vec.push(x);
    }
    //println!("in_vec is {:?}",in_vec);
    //
    //Now we start the challenge:
    //Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    //First, Median.

    in_vec.sort();
    //println!("Sorted in_vec is {:?}",in_vec);
    println!("The Randomly generated Vector has been sorted, and it's length is {}", in_vec.len());
    //now, median is middle val... out of 15 though?
    // A ha, school level textbook to the rescue:
    // If the number of observations is odd, the number in the middle of the list is the median
    // For even numbers, you take an average of the two middle values so 1,2,3,4 would be (2+3)/2
    // let median  = in_vec.get((in_vec.len()+1)/2);  <-- this doesn't work as Vecs are 0
    // indexed unlike real life
    let opt_median  = in_vec.get(in_vec.len()/2);
    // I feel like there must be a more elegant way of doing this than seperate variables for the
    // option and the float.
    let mut median :f32;
    match opt_median  {
        Some(i) => {
            median = *i as f32;
            // if the vector is an even length, get the len/2-1, add it to the current median and
            // divide to get the actual median.
            if in_vec.len()%2 == 0 {
                let other_val_opt = in_vec.get(in_vec.len()/2-1);
                let other_val: f32;
                match other_val_opt {
                    Some(j) => {
                        other_val = *j as f32;
                        median = (median+other_val)/2.0;
                    }
                    None => panic!("Couldn't get other val of even vec length for median calculation"),
                };
            }
        }
        None => panic!("Couldn't get the median of in_vec: {:?}", in_vec),
    };
    println!("The median of Sorted in_vec is {}", median);
    //
    println!("-----------------------------");
    println!("Getting the Mode by adding the values to a hashmap and finding the most common value. This one was kind of made easy by the fact that the code you need for it is literaly on the same page as the challenge :D");
    println!("-----------------------------");
    let mut mode_map = HashMap::new();
    for num in in_vec{
        let count = mode_map.entry(num).or_insert(0);
        *count+=1;
    }
//    println!("Mode Hashmap: {:?}",mode_map);
    let mut max_pair = (0,0);
    for (key,val) in mode_map {
       if val > max_pair.1 {
            max_pair = (key,val);
            

//            println!("Currently {} is the mode with {} entries",key,val);
       }
       else{

       }
    }
    println!("{} is the mode with {} entries",max_pair.0,max_pair.1);
    println!("Note that the source code debugs many values I've left out here for brevity. It's also probably ugly as sin but that's something to fix later");
}

fn pig_latin(){
    println!("-----------------------------");
    println!("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!");
    println!("-----------------------------");
    
    println!("Ok so I need some words, lets use the words from the challenge, as it adds the extra challenge of handling the quotes etc.");
    println!("-----------------------------");
    let words = String::from("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!");
    println!("Words are {}",words);
    // So, first we need to collect the words.
    let word_vec : Vec<_>= words.split_whitespace().collect();
    println!("word_vec is {:?}", word_vec);
    // then we need to be able to access the length of each word and it's first letter
    for word in word_vec {
        println!("Word {} is {} characters long", word, word.len());
        // i feel like an iterator makes more sense here so I can specify only taking the first
        // char. It'll help with other character checking too
        for c in word.chars(){
            println!("First char in {} is {}", word, c);
    // Now you need to check if first letter is consonant or vowel, or neither, and handle it
            if c.is_alphanumeric(){
                // match vowels and consants here
                println!("TODO: make thie make sense: {}", pig_up(word.to_string()));
            }
            else {
                // check the next letter to see if it's actually a letter
                // Also, if the end of the word is a ! or ? or ) " any of these, it'll need moving
                // to the new end. Handle all edge cases.
            }
            //this isn't the best way to do this but it works for now, iterator should replace it
            break;
        }
    }
    
    // Then spit the Vec back out into a string.



}
    // Then move the correct letters (or not if it's a vowel) to the end with the -ay/-hay ting
fn pig_up(input: String) -> String {
    let outstr;
    match &input[0..1]{
        "a"|"e"|"i"|"o"|"u" => outstr = String::from(input[..]+"-hay".as_str()),
        _ => println!("You consonating."),
    };
    let bing = String::from("temp");
    let ting = input+&bing; 
    ting
}

fn employees(){
    unimplemented!();
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 || args.get(1).unwrap() == "-h" {
        println!("Usage: -i for Integer Median Mode, -p for Pig Latin, -e for Employees");
        process::exit(1);
    }
//match the args to the modes
    match args[1].as_str(){
        "-i" => {
            println!("Integer Mode/Median Mode");
            int_med_mode();
        },
        "-p" => {
            println!("Pig Latin Mode");
            pig_latin();
        },
        "-e" => {
            println!("Employee Text Interface Mode");
            employees();

        },
        _ => panic!("Invalid input. Usage: -i for Integer Median Mode, -p for Pig Latin, -e for Employees"),
        
    };
    
}

