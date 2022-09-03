use std::collections::HashMap;

fn main() {
    let basketball_players = parse("Jill|Huang|Gators,Janko|Barton|Sharks,Wanda|Vakulskas|Sharks,Jill|Moloney|Gators,Luuk|Watkins|Gators");
    let football_players = parse("Hanzla|Radosti|32ers,Tina|Watkins|Barleycorns,Alex|Patel|32ers,Jill|Huang|Barleycorns,Wanda|Vakulskas|Barleycorns");
    let in_both: Option<Vec<&str>> = both_sports(basketball_players, football_players);
    if let Some(people) = in_both {
        println!("In both sports: {}", people.join(", "));
    } else {
        println!("No one is in both sports!")
    }

    let my_vector: Vec<i32> = vec![8, 2, 3, 9, 4, 7, 5, 0, 6];
    let missing = missing_num(my_vector);
    println!("{missing}");

    let stock_prices: Vec<i32> = vec![4, 5, 1, 8, 6, 3, 9];
    stock_price_diff(stock_prices);
}

// O(N + M) implementation;
fn both_sports<'a>(
    basketball_players: Vec<HashMap<&str, &'a str>>,
    football_players: Vec<HashMap<&str, &'a str>>,
) -> Option<Vec<&'a str>> {
    let mut to_return: Vec<&'a str> = vec![];
    // Use basketball players as a HashMap. Iter through football players. Utilize magical lookups.
    let mut bp_hash: HashMap<&str, bool> = HashMap::new();

    for bplayer in basketball_players.iter() {
        bp_hash.insert(bplayer["first_name"], true);
    }

    for fplayer in football_players.iter() {
        let first_name = fplayer["first_name"];
        if bp_hash.contains_key(first_name) {
            to_return.push(first_name);
        }
    }

    if !to_return.is_empty() {
        Some(to_return)
    } else {
        None
    }
}

// We can consider a very "weird" solution where, after finding out the maximum & minimum
// number in the vector, we iterate through the vector and check:
// a. assuming n is not the max or min number; n + 1 exists within the array; n - 1 exists within the array
// b. assuming n is the max number; n - 1 exists within the array
// c. assuming n is the min number; n + 1 exists within the array
fn missing_num(nums: Vec<i32>) -> i32 {
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();

    for num in nums.iter() {
        if num != max && num != min {
            if !nums.contains(&(num + 1)) {
                return num + 1;
            } else if !nums.contains(&(num - 1)) {
                return num - 1;
            }
        } else if num == max {
            if !nums.contains(&(num - 1)) {
                return num - 1;
            }
        } else if num == min {
            if !nums.contains(&(num + 1)) {
                return num + 1;
            }
        }
    }

    0
}

fn stock_price_diff(prices: Vec<i32>) -> (i32, i32) {
    let mut min = f64::INFINITY;
    let mut max = 0;

    for price in prices.iter() {
        if 
    }

    (buy, sell)
}

fn parse(input_str: &str) -> Vec<HashMap<&str, &str>> {
    let mut to_return: Vec<HashMap<&str, &str>> = vec![];

    let maps: Vec<&str> = input_str.split(',').collect();
    let mut values: Vec<Vec<&str>> = vec![];

    for map in maps.iter() {
        let to_push = map.split('|').collect();
        values.push(to_push);
    }

    for val in values.iter() {
        to_return.push(HashMap::from([
            ("first_name", val[0]),
            ("last_name", val[1]),
            ("team", val[2]),
        ]));
    }

    to_return
}
