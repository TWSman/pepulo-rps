#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;
use std::collections::BTreeMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use rand::Rng;
use std::fmt::Display;
use core::fmt;
use log::info;

#[derive(Debug, Clone)]
pub struct Game {
    pub player_list: BTreeMap<u16, Player>,
    pub match_list: BTreeMap<(u16, u16), Match>,
    queue: PriorityQueue<(u16, u16), i64>,
    rng_seed: usize,
}

fn get_quote(i: usize) -> (String, String) {
    let quotes: Vec<(&str, &str)> = vec![
        ("Victory belongs to the most persevering.", "Napoleon Bonaparte"),
        ("Your victory is right around the corner. Never give up.", "Nicky Minaj"),
        ("War is a series of catastrophes which result in victory.", "Albert Pike"),
        ("Know thy self, know thy enemy. A thousand battles, a thousand victories", "Sun Tzu"),
        ("No victory without suffering", "J. R. R. Tolkien"),
        ("Victory comes from finding opportunities in problems", "Sun Tzu"),
        ("Without training, they lacked knowledge. Without knowledge, they lacked confidence. Without confidence, they lacked victory.", "Julius Ceasar"),
        ("The will to conquer is the first condition of victory", "Ferdinand Foch"),
        ("The more difficult the victory, the greater the happiness in winning", "Pele"), //TODO!
        ("Preparedness is the key to success and victory", "Douglas MacArthur"),
        // ("It’s always darkest before the dawn. The bigger your challenge, the closer you are to your victory.", "Joel Osteen"),
        ("So a military force has no constant formation, water has no constant shape: the ability to gain victory by changing and adapting according to the opponent is called genius.", "Sun Tzu"),
        ("Even the smallest victory is never to be taken for granted. Each victory must be applauded...", "Audre Lorde"),
        ("There is only one decisive victory: the last.","Carl von Clausewitz"),
        ("Victory is sweetest when you've known defeat.", "Malcolm Forbes"),
        ("You ask what the aim is? I tell you it is victory - total victory.", "Winston Churchill "),
        ("Forewarned, forearmed; to be prepared is half the victory.","Miguel de Cervantes"),
        ("Full effort is full victory.","Mahatma Gandhi"),
        ("If there exists no possibility of failure, then victory is meaningless.","Robert H. Schuller"),
        ("We lost because we told ourselves we lost.","Leo Tolstoy"),
        ("The secret of all victory lies in the organization of the non-obvious.", "Marcus Aurelius"),
        ("I would challenge you to a battle of wits, but I see you are unarmed!","William Shakespeare"),
        ("Sometimes by losing a battle you find a new way to win the war.","Donald Trump"),
        ("All men can see these tactics whereby I conquer, but what none can see is the strategy out of which victory is evolved.","Sun Tzu"),
        ("Our greatest glory is not in never falling, but in rising every time we fall.", "Confucius"),
        ("Somebody's gotta win and somebody's gotta lose and I believe in letting the other guy lose.", "Pete Rose"),
    ];
    let q = quotes[i % quotes.len()];
    (q.0.to_string(), q.1.to_string())
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_list: BTreeMap::new(),
            match_list: BTreeMap::new(),
            queue: PriorityQueue::new(),
            rng_seed: rand::thread_rng().gen_range(0..100),
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

    pub fn get_next_game(&self) -> Option<&Match> {
        if self.queue.iter().all(|(k,p)| p < &0) {
            None
        } else {
            let (i,_p) = self.queue.peek().unwrap();
            self.match_list.get(&i)
        }
        //self.queue.clone().into_sorted_iter().filter_map(|(k, prior)| {
        //    if prior > 0 {
        //        Some(self.match_list.get(&k).unwrap())
        //    } else {
        //        None
        //    }
        //}).take(n).collect::<Vec<_>>().clone()
    }

    pub fn get_played_n(&self) -> usize {
        self.queue.clone().into_sorted_iter().filter(|(_k, prior)|  {
            prior < &0
        }).count()
    }

    pub fn get_quote(&self) -> (String, String) {
        let i = self.get_played_n();
        get_quote(i + self.rng_seed)
    }
    pub fn get_played_games(&self) -> Vec<(&Match, i64)> {
        self.queue.clone().into_sorted_iter().filter_map(|(k, prior)|  {
            if prior < 0 {
                Some((self.match_list.get(&k).unwrap(), prior))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }

    pub fn add_player(&mut self, name: &str) -> Result<(), String> {
        if self.player_list.values().filter(|p| p.name == name).count() > 0 {
            return Err("Player Already exists".to_string());
        }
        let id = self.player_list.keys().max().unwrap_or(&0) + 1;
        let player: Player = Player::new(name, id);
        let mut i = 0;
        for (id, p) in &self.player_list {
            if self.match_list.contains_key(&(p.id, player.id)) {
                return Err("Match Already exists".to_string());
            }
            self.match_list.insert(
                (*id, player.id), Match::new(p, &player, i)
            );
            i += 1;
            self.queue.push((*id, player.id), 0);
        }
        self.player_list.insert(player.id, player);
        self.update_priorities();
        Ok(())
    }

    pub fn add_result(&mut self, game_id: (u16, u16), play1: Rps, play2: Rps) {
        info!("Adding result to game");
        let m = self.match_list.get_mut(&game_id).unwrap();
        let player1 = self.player_list.get_mut(&m.player1).unwrap();
        let result = play1.result(&play2);
        let score = result.get_score();
        let player1_score = play1.get_score() + score;
        let player2_score = play2.get_score() + (6 - score);
        player1.played += 1;
        player1.score += player1_score;
        info!("Player {} has played {} games with a total score {}", player1.name, &player1.played, &player1.score);
        m.play1 = Some(play1);
        m.play2 = Some(play2);
        m.result = Some(result);
        let player2 = self.player_list.get_mut(&m.player2).unwrap();
        player2.played += 1;
        player2.score += player2_score;
        info!("Player {} has played {} games with a total score {}", player2.name, &player2.played, &player2.score);
        self.update_priorities();
    }

    pub fn update_priorities(&mut self) {
        info!("Updating priorities");
        if self.player_list.is_empty() {
            info!("Player List is empty");
            return;
        }
        let played_games = self.player_list.values().map(|p| p.played as i64).sum::<i64>() / 2 + 1;
        let n_games = (self.player_list.len() - 1) as u16;
        for (k, m) in &self.match_list {
            info!("Priority for game {} - {}", k.0, k.1);
            let player1 = match self.player_list.get(&m.player1) {
                Some(p) => p,
                None => {
                    info!("Error getting {}", m.player1);
                    panic!("");
                }
            };
            let player2 = self.player_list.get(&m.player2).unwrap();
            info!("\t{} - {}", player1.name, player2.name);
            match self.queue.get(k) {
                Some((_i,p)) if p < &0 => {
                    info!("Game already has negative priority");
                    continue;
                },
                _ => (),
            }

            if m.play1.is_some() & m.play2.is_some() {
                info!("\tSet negative priority");
                self.queue.change_priority(k, -played_games);
                continue;
            }
            let round = m.round;
            info!("\tRound = {}", &round);
            let games_left1 = n_games - player1.played;
            let games_left2 = n_games - player2.played;
            info!("\tGames left = {} / {}", &games_left1, &games_left2);
            let potential1 = games_left1 * 9;
            let potential2 = games_left2 * 9;
            info!("\tPotential = {} / {}", &potential1, &potential2);
            let score1 = player1.score;
            let score2 = player2.score;
            info!("\tScore = {} / {}", &score1, &score2);
            let priority = potential1 + score1 + potential2 + score2 + round;
            info!("\tPrioritye = {}", &priority);
            self.queue.change_priority(k, (9999 - priority).into());
        }
        info!("priorities updated");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Match {
    pub player1: u16, 
    pub player2: u16,
    pub play1: Option<Rps>,
    pub play2: Option<Rps>,
    pub result: Option<RpsResult>,
    round: u16,
}

impl Match {
    fn new(player1: &Player, player2: &Player, round: u16) -> Self {
        Match {
            player1: player1.id,
            player2: player2.id,
            play1: None,
            play2: None,
            result: None,
            round,
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum RpsResult {
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

impl Display for Rps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
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
            Rps::Rock => "🪨",
            Rps::Paper => "📜",
            Rps::Scissors => "✂️",
        }
    }


    pub fn new(inp: &str) -> Self {
        match inp {
            "🪨" => Self::Rock,
            "📜" => Self::Paper,
            "✂️" => Self::Scissors,
            _ => panic!("Unknown string"),
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
        let _ = game.add_player("Alice");
        let _ = game.add_player("Bob");
        let _ = game.add_player("Charlie");

        game.add_result((1,2), Rps::Rock, Rps::Scissors);
        game.add_result((1,3), Rps::Rock, Rps::Paper);

        let _ = game.add_player("David");
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
