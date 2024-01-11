#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;
use std::collections::BTreeMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug)]
pub struct Game {
    player_list: BTreeMap<u16, Player>,
    match_list: BTreeMap<(u16, u16), Match>,
    queue: PriorityQueue<(u16, u16), i64>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_list: BTreeMap::new(),
            match_list: BTreeMap::new(),
            queue: PriorityQueue::new(),
        }
    }

    pub fn get_player_name(&self, pid: u16) -> String {
        self.player_list.get(&pid).unwrap().name.clone()
    }

    pub fn get_player(&self, pid: u16) -> Player {
        self.player_list.get(&pid).unwrap().clone().to_owned()
    }

    pub fn get_scores(&self) -> Vec<&Player> {
        let mut sorted_scores = self.player_list.values().collect::<Vec<_>>();
        sorted_scores.sort_by_key(|p1| Reverse(p1.score));
        sorted_scores

    }

    pub fn get_next_games(&self, n: usize) -> Vec<&Match> {
        self.queue.clone().into_sorted_iter().filter_map(|(k, prior)| {
            if prior > 0 {
                Some(self.match_list.get(&k).unwrap())
            } else {
                None
            }
        }).take(n).collect::<Vec<_>>().clone()
    }

    pub fn get_played_games(&self) -> Vec<&Match> {
        self.queue.iter().filter_map(|(k, prior)|  {
            if prior < &0 {
                Some(self.match_list.get(k).unwrap())
            } else {
                None
            }
        }).collect::<Vec<&Match>>()
    }

    pub fn add_player(&mut self, name: &str) {
        let id = self.player_list.keys().max().unwrap_or(&0) + 1;
        let player: Player = Player::new(name, id);
        for (id, p) in &self.player_list {
            if self.match_list.contains_key(&(p.id, player.id)) {
                panic!("Match already exists");
            }
            self.match_list.insert(
                (*id, player.id), Match::new(p, &player)
            );
            self.queue.push((*id, player.id), 0);
        }
        self.player_list.insert(player.id, player);
        self.update_priorities();
    }

    pub fn add_result(&mut self, game_id: (u16, u16), play1: Rps, play2: Rps) {
        let m = self.match_list.get_mut(&game_id).unwrap();
        let player1 = self.player_list.get_mut(&m.player1).unwrap();
        let result = play1.result(&play2).get_score();
        let player1_score = play1.get_score() + result;
        let player2_score = play2.get_score() + (6 - result);
        player1.played += 1;
        player1.score += player1_score;
        m.play1 = Some(play1);
        m.play2 = Some(play2);
        let player2 = self.player_list.get_mut(&m.player2).unwrap();
        player2.played += 1;
        player2.score += player2_score;
        self.update_priorities();
    }

    pub fn update_priorities(&mut self) {
        if self.player_list.is_empty() {
            return;
        }
        let played_games = self.player_list.values().map(|p| p.played as i64).sum::<i64>() / 2;
        let n_games = (self.player_list.len() - 1) as u16;
        for (k, m) in &self.match_list {
            if m.play1.is_some() & m.play2.is_some() {
                self.queue.change_priority(k, -played_games);
                continue;
            }
            let player1 = self.player_list.get(&m.player1).unwrap();
            let player2 = self.player_list.get(&m.player2).unwrap();
            let games_left1 = n_games - player1.played;
            let games_left2 = n_games - player2.played;
            let potential1 = games_left1 * 9;
            let potential2 = games_left2 * 9;
            let score1 = player1.score;
            let score2 = player2.score;
            let priority = potential1 - score1 + potential2 - score2;
            self.queue.change_priority(k, priority.into());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Match {
    pub player1: u16, 
    pub player2: u16,
    pub play1: Option<Rps>,
    pub play2: Option<Rps>,
}

impl Match {
    fn new(player1: &Player, player2: &Player) -> Self {
        Match {
            player1: player1.id,
            player2: player2.id,
            play1: None,
            play2: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u16,
    pub name: String,
    pub score: u16,
    pub played: u16,
}

#[allow(dead_code)]
impl Player {
    fn new(name: &str, id: u16) -> Player {
        Self {name: name.to_string(), id, score: 0, played: 0}
    }
}


#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum RpsResult {
    Win,
    Lose,
    Draw,
}

#[allow(dead_code)]
impl RpsResult {
    fn get_score(&self) -> u16 {
        match self {
            RpsResult::Win => 6,
            RpsResult::Draw => 3,
            RpsResult::Lose => 0,
        }
    }
}

#[allow(dead_code)]
impl Rps {
    fn get_score(&self) -> u16 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    pub fn str(&self) -> &str {
        match self {
            Rps::Rock => "Kivi 🪨",
            Rps::Paper => "Paperi 📜",
            Rps::Scissors => "Sakset ✂️",
        }
    }

    fn result(&self, other: &Rps) -> RpsResult {
        if self == other {
            RpsResult::Draw
        } else if other == &self.win() {
            RpsResult::Lose
        } else {
            RpsResult::Win
        }
    }

    fn win(&self) -> Rps {
        FromPrimitive::from_u8((*self as u8 + 1) % 3).unwrap()
    }

    fn lose(&self) -> Rps {
        FromPrimitive::from_u8((*self as u8 + 2) % 3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps() {
        let rock = Rps::Rock;
        let paper = Rps::Paper;
        let scissors = Rps::Scissors;
        assert_eq!(rock.result(&rock), RpsResult::Draw);
        assert_eq!(rock.result(&paper), RpsResult::Lose);
        assert_eq!(rock.result(&scissors), RpsResult::Win);
        assert_eq!(paper.result(&scissors), RpsResult::Lose);
        assert_eq!(paper.result(&rock), RpsResult::Win);
        assert_eq!(scissors.result(&rock), RpsResult::Lose);
        assert_eq!(scissors.result(&paper), RpsResult::Win);
    }

    #[test]
    fn new_player_midgame() {
        let mut game = Game::new();
        game.add_player("Alice");
        game.add_player("Bob");
        game.add_player("Charlie");

        game.add_result((1,2), Rps::Rock, Rps::Scissors);
        game.add_result((1,3), Rps::Rock, Rps::Paper);

        game.add_player("David");
        game.update_priorities();
        assert_eq!(game.player_list.len(), 4);
        assert_eq!(game.match_list.len(), 6);
        assert_eq!(game.queue.len(), 6);

        let p1 = game.player_list.get(&1).unwrap();
        let p2 = game.player_list.get(&2).unwrap();
        let p3 = game.player_list.get(&3).unwrap();
        let p4 = game.player_list.get(&4).unwrap();
        assert_eq!(p1.score, 8);
        assert_eq!(p2.score, 3);
        assert_eq!(p3.score, 8);
        assert_eq!(p4.score, 0);

        assert_eq!(p1.played, 2);
        assert_eq!(p2.played, 1);
        assert_eq!(p3.played, 1);
        assert_eq!(p4.played, 0);

        let played_games = game.get_played_games();
        assert_eq!(played_games.len(), 2);

        dbg!(&game.queue);
        let (g3, p) = game.queue.pop().unwrap();
        assert_eq!(g3, (2,4));

        let scores = game.get_scores();
        assert_eq!(scores[0].id, 1);
        assert_eq!(scores[1].id, 3);
        assert_eq!(scores[2].id, 2);
        assert_eq!(scores[3].id, 4);
    }

    #[test]
    fn game() {
        let mut game = Game::new();
        game.add_player("Alice");
        assert_eq!(game.player_list.len(), 1);
        assert_eq!(game.match_list.len(), 0);
        assert_eq!(game.queue.len(), 0);
        game.add_player("Bob");
        assert_eq!(game.player_list.len(), 2);
        assert_eq!(game.match_list.len(), 1);
        assert_eq!(game.queue.len(), 1);

        game.add_player("Charlie");
        assert_eq!(game.player_list.len(), 3);
        assert_eq!(game.match_list.len(), 3);
        assert_eq!(game.queue.len(), 3);

        //game.update_priorities();

        game.add_result((1,2), Rps::Rock, Rps::Scissors);
        let p1 = game.player_list.get(&1).unwrap();
        let p2 = game.player_list.get(&2).unwrap();
        assert_eq!(p1.score, 7);
        assert_eq!(p2.score, 3);

        assert_eq!(p1.played, 1);
        assert_eq!(p2.played, 1);
        //game.update_priorities();
        dbg!(&game.queue);
        dbg!(&game.player_list);
        let (g3, p) = game.queue.pop().unwrap();
        // 2,3 should come first since player 1 has a higher score
        // Thus 1,3 has lower priority than 2,3 
        assert_eq!(g3, (2,3));
        let (g3, p) = game.queue.pop().unwrap();
        assert_eq!(g3, (1,3));

        let scores = game.get_scores();
        //dbg!(&game.queue);
        let played_games = game.get_played_games();
        dbg!(&played_games);
        assert_eq!(played_games.len(), 1);
        dbg!(&scores);
        assert_eq!(scores[0].id, 1);
        assert_eq!(scores[1].id, 2);
        assert_eq!(scores[2].id, 3);

        // #3 wins, gets 6 + 2 points, 1 point for #1
        game.add_result((1,3), Rps::Rock, Rps::Paper);
        let p1 = game.player_list.get(&1).unwrap();
        let p3 = game.player_list.get(&3).unwrap();
        assert_eq!(p1.score, 8);
        assert_eq!(p3.score, 8);

        assert_eq!(p1.played, 2);
        assert_eq!(p3.played, 1);

        let played_games = game.get_played_games();
        assert_eq!(played_games.len(), 1);

        let scores = game.get_scores();
        dbg!(&scores);
        assert_eq!(scores[0].id, 1);
        assert_eq!(scores[1].id, 3);
        assert_eq!(scores[2].id, 2);
    }
}
