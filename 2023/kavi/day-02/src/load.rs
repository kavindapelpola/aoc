pub fn load() -> Vec<Vec<(u32, u32, u32)>> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    for line in input.lines() {
        let games = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        let mut games_vec = vec![];
        for game in games {
            let dice = game.split(",").collect::<Vec<&str>>();
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for die in dice {
                let r = die.trim().split(" ").collect::<Vec<&str>>();
                match r[1] {
                    "red" => red = r[0].parse::<u32>().unwrap(),
                    "green" => green = r[0].parse::<u32>().unwrap(),
                    "blue" => blue = r[0].parse::<u32>().unwrap(),
                    _ => panic!("Invalid color"),
                }
            }
            games_vec.push((red, blue, green));
        }
        res.push(games_vec);
    }
    res
}
