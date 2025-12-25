use std::{
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    i32,
};

fn main() {
    println!("Hello, world {}!", 'z' as usize);
}
struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let num_pad = HashMap::from([
            (2, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z']),
        ]);
        let digits: Vec<usize> = digits
            .chars()
            .map(|a| {
                let a = a.to_string();
                a.parse::<usize>().unwrap()
            })
            .collect();
        let mut current_path: Vec<char> = Vec::new();
        let mut all_paths: Vec<Vec<char>> = Vec::new();
        let combinations = num_pad.get(&digits[0]).unwrap();
        for combination in combinations {
            Solution::find_paths(
                *combination,
                &digits,
                1,
                &num_pad,
                &mut current_path,
                &mut all_paths,
            );
        }
        all_paths
            .into_iter()
            .map(|a| String::from_iter(a.into_iter()))
            .collect()
    }
    fn find_paths(
        character_to_add: char,
        digits: &[usize],
        index: usize,
        num_pad: &HashMap<usize, Vec<char>>,
        current_path: &mut Vec<char>,
        all_paths: &mut Vec<Vec<char>>,
    ) {
        current_path.push(character_to_add);
        if index >= digits.len() {
            all_paths.push(current_path.clone());
            current_path.pop();
            return;
        }
        let combinations = num_pad.get(&digits[index]).unwrap();

        for combination in combinations {
            Solution::find_paths(
                *combination,
                digits,
                index + 1,
                num_pad,
                current_path,
                all_paths,
            );
        }
        current_path.pop();
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut word_characters: Vec<char> = word.chars().collect();
        let rows = board.len();
        let columns = board[0].len();
        let mut board = board;
        let mut word_letter_count = HashMap::new();
        let mut board_letter_count = HashMap::new();
        if rows == 1 && columns == 1 && word_characters.len() == 1 {
            return word_characters[0] == board[0][0];
        }
        for i in 0..rows {
            for j in 0..columns {
                board_letter_count
                    .entry(board[i][j])
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
        for letter in word_characters.iter() {
            word_letter_count
                .entry(*letter)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for (key, value) in word_letter_count {
            match board_letter_count.get(&key) {
                Some(count) => {
                    if *count < value {
                        return false;
                    }
                }
                None => return false,
            }
        }

        if board_letter_count.get(&word_characters[0]).unwrap()
            > board_letter_count
                .get(&word_characters[word_characters.len() - 1])
                .unwrap()
        {
            word_characters.reverse();
        }

        for i in 0..rows {
            for j in 0..columns {
                if Solution::floodfill(i as i32, j as i32, &mut board, 0, &word_characters) {
                    return true;
                }
            }
        }
        false
    }

    pub fn floodfill(
        row: i32,
        column: i32,
        board: &mut [Vec<char>],
        index_of_char: usize,
        word_characters: &[char],
    ) -> bool {
        if index_of_char == word_characters.len() {
            return true;
        }
        if board[row as usize][column as usize] != word_characters[index_of_char] {
            return false;
        }
        let previous_character = board[row as usize][column as usize];
        board[row as usize][column as usize] = '#';
        let possible_directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        for direction in possible_directions {
            let row_index = row + direction.0;
            let column_index = column + direction.1;
            if row_index >= 0
                && row_index < board.len() as i32
                && column_index >= 0
                && column_index < board[0].len() as i32
            {
                if Solution::floodfill(
                    row_index,
                    column_index,
                    board,
                    index_of_char + 1,
                    word_characters,
                ) {
                    return true;
                }
            }
        }
        board[row as usize][column as usize] = previous_character;
        false
    }
}
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let p_len = p.len();
        let s_len = s.len();
        if p_len > s_len {
            return vec![];
        }
        let mut pos_anagram = Vec::new();
        let mut original_map = HashMap::new();

        for letter in p {
            original_map
                .entry(letter)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut s_map = HashMap::new();

        for letter in &s[0..p_len] {
            s_map
                .entry(*letter)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        if Solution::simple_anagram(&s_map, &original_map) {
            pos_anagram.push(0);
        }
        for i in p_len..s_len {
            if Solution::is_anagram(&mut s_map, s[i], s[i - p_len], &original_map) {
                pos_anagram.push((i - (p_len - 1)) as i32);
            }
        }
        pos_anagram
    }
    pub fn simple_anagram(part_map: &HashMap<char, i32>, original: &HashMap<char, i32>) -> bool {
        for (key, value) in original {
            match part_map.get(key) {
                Some(count) => {
                    if *count != *value {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_anagram(
        part_map: &mut HashMap<char, i32>,
        incoming: char,
        outgoing: char,
        original: &HashMap<char, i32>,
    ) -> bool {
        match part_map.get_mut(&outgoing) {
            Some(count) => {
                *count -= 1;
            }
            None => {
                return false;
            }
        }
        part_map
            .entry(incoming)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        for (key, value) in original {
            match part_map.get(key) {
                Some(count) => {
                    if *count != *value {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n==1{
            return vec![0];
        }
        let mut n=n;
        
        let mut edge_count=vec![0;n as usize];
        let adjacent_list=Solution::create_adjacency_list(n as usize,edges);
        let mut labels=Vec::new();
        let mut leaves=VecDeque::new();
        for (label,neighbors) in adjacent_list.iter().enumerate(){
            let neighbors_count=neighbors.len();
            if neighbors_count==1{
                leaves.push_back(label as i32);
            }
            edge_count[label]=neighbors_count;
            
        }
        while !leaves.is_empty(){
            if n<=2{
                labels.extend(leaves.iter());
                return labels;
                
            }
            for _ in 0..leaves.len(){
                n-=1;
                let node=leaves.pop_front().unwrap();
                let neighbours=&adjacent_list[node as usize];
                for neighbor in neighbours{
                    edge_count[*neighbor as usize]-=1;
                    if edge_count[*neighbor as usize]==1{
                        leaves.push_back(*neighbor);
                    }
                }
            }
        }
        

        labels
    }
    fn create_adjacency_list(n:usize,edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacency_list = vec![vec![];n];
        for edge in edges{
            adjacency_list[edge[0] as usize].push(edge[1]);
            adjacency_list[edge[1] as usize].push(edge[0]);
        }
        adjacency_list
    }
    fn bfs(root: i32, adjacency_list: &HashMap<i32, Vec<i32>>,minimum:&i32) -> i32 {
        let mut height = 0;
        let mut queue = VecDeque::new();
        let mut seen=HashSet::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let current_len = queue.len();
            for _ in 0..current_len {
                let current_item = queue.pop_front().unwrap();
                seen.insert(current_item);
                let neighbours = adjacency_list.get(&current_item).unwrap();
                for neighbour in neighbours {
                    if !seen.contains(neighbour) {
                        queue.push_back(*neighbour);
                    }
                }
            }
            height += 1;
            if height>*minimum{
                return i32::MAX;
            }
        }
        height
    }
}
