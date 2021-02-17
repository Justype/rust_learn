use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10u32);

    let ey = scores.entry("黄队".to_string());
    println!("{:?}", ey);   // Entry(VacantEntry("黄队"))
    ey.or_insert(50);   // Entry(OccupiedEntry { key: "蓝队", value: 10 })

    let eb = scores.entry("蓝队".to_string());
    println!("{:?}", eb);
    eb.or_insert(20);
    println!("{:?}", scores);

    // let teams = vec![String::from("蓝队"), String::from("黄队")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // println!("{:?}", scores);

    print_score(&scores, &"蓝队".to_string());
    print_score(&scores, &"红队".to_string());
    
}

fn print_score(scores: &HashMap<String, u32>, team_name: &String){
    match scores.get(team_name) {
        Some(s) => println!("{}的分数是{}", team_name, s),
        None => println!("无法找到{}的成绩", team_name),
    }
}
