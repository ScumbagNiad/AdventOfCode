extern crate reqwest;

fn main() {
    let session = "53616c7465645f5f9a1b0278e9004844d5fb7fe48e465ab29dffcb8fab613394b88ebb447ca05b4b640daa02786d075c"; 
    let session_cookie = "session=".to_owned()+session;

    let client = reqwest::Client::new();
    let request = client.get("https://adventofcode.com/2018/day/1/input")
        .header("cookie",session_cookie)
        .send()
        .unwrap()
        .text()
        .unwrap();
    
    let split = request.split("\n");
    let numbers = split.collect::<Vec<&str>>();

    let mut results = Vec::new();
    let mut no_duplicates = true;

    let mut answer = 0;
    while no_duplicates {

        for x in 0..numbers.len() {
            if numbers[x] != "" {
                answer = answer + numbers[x].parse::<i32>().unwrap();

                if results.contains(&answer) {
                    println!("Duplicate {}", answer);
                    no_duplicates = false;
                    break;
                }
                results.push(answer);
            }
        }

    }
    
    println!("{}", answer);
}