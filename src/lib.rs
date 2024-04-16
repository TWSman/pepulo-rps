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
use log::debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameMode {
    RPS,
    RPSSL,
}

impl GameMode {
    pub fn str(&self) -> &str {
        match self {
            Self::RPS => "RPS",
            Self::RPSSL => "RPSSL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub player_list: BTreeMap<u16, Player>,
    // keys are player1, player2, round
    pub match_list: BTreeMap<(u16, u16, u16), Match>,
    queue: PriorityQueue<(u16, u16, u16), i64>,
    rng_seed: usize,
    rounds: usize,
    game_mode: GameMode,
}


impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_list: BTreeMap::new(),
            match_list: BTreeMap::new(),
            queue: PriorityQueue::new(),
            rng_seed: rand::thread_rng().gen_range(0..100),
            rounds: 1,
            game_mode: GameMode::RPS,
        }
    }

    pub fn get_options(&self) -> Vec<Rpssl> {
        Rpssl::iter().collect::<Vec<_>>()
    }

    pub fn get_player_name(&self, pid: u16) -> Option<String> {
        Some(self.player_list.get(&pid)?.name.clone())
    }

    pub fn get_player(&self, pid: u16) -> Option<Player> {
        Some(self.player_list.get(&pid)?.clone().to_owned())
    }

    fn update_scores(&mut self) {
        let keys = self.player_list.keys().map(|m| m.to_owned()).collect::<Vec<u16>>();
        for p1 in keys {
            let (played, score) = self.get_score_for_player(p1);

            let player = self.player_list.get_mut(&p1).expect("Key should exist");
            player.played = played;
            player.score = score;
        }
    }
    pub fn get_scores(&mut self) -> Vec<&Player> {
        self.update_scores();
        let mut sorted_scores = self.player_list.values().collect::<Vec<_>>();
        sorted_scores.sort_by_key(|p1| Reverse(p1.score));
        sorted_scores
    }

    fn get_score_for_player(&self, pid: u16) -> (u16, u16) {
        let matches = self.match_list.iter().filter(|((p1,p2,_round), m)| {
            ((p1 == &pid) | (p2 == &pid)) & (m.result.is_some())
        }).collect::<Vec<_>>();
        (matches.len() as u16, 
            matches.iter().map(|(_,m)| m.get_score_for_player(pid)).sum()
        )
    }

    pub fn set_mode(&mut self, game_mode: GameMode) -> Result<(), String>{
        if self.get_played_n() > 0 {
            Err(String::from("Remove played games before changing game mode"))
        } else {
            self.game_mode = game_mode;
            Ok(())
        }
    }

    pub fn get_mode(&self) -> GameMode {
        self.game_mode.clone()
    }

    pub fn set_rounds(&mut self, rounds: usize) {
        let old_rounds = self.rounds;
        self.rounds = rounds;
        if rounds > old_rounds {
            let ids = self.player_list.keys().collect::<Vec<_>>();
            for player_id in ids {
                let player = self.player_list.get(player_id).expect("Key should exist");
                for (id, p) in &self.player_list {
                    if id <= player_id {
                        continue;
                    }
                    for round in (old_rounds+1)..=(self.rounds) {
                        if self.match_list.contains_key(&(p.id, *player_id, round as u16)){
                            info!("Match Already exists");
                        }
                        self.match_list.insert(
                            (*id, *player_id, round as u16), Match::new(p, player, round as u16)
                        );
                        self.queue.push((*id, player.id, round as u16), 0);
                    }
                }
            }
        }
    }

    pub fn get_rounds(&mut self) -> usize {
        self.rounds
    }

    pub fn get_next_games(&self, n: usize) -> Vec<&Match> {
        self.queue.clone().into_sorted_iter().filter_map(|(k, prior)| {
            if prior > 0 {
                Some(self.match_list.get(&k).expect("Match list should include all queue elements"))
            } else {
                None
            }
        }).take(n).collect::<Vec<_>>().clone()
    }

    pub fn get_next_game(&self) -> Option<&Match> {
        if self.queue.iter().all(|(_k,p)| p < &0) {
            None
        } else {
            let (i,_p) = self.queue.peek()?;
            self.match_list.get(i)
        }
    }

    pub fn get_played_n(&self) -> usize {
        self.queue.clone().into_sorted_iter().filter(|(_k, prior)|  {
            prior < &0
        }).count()
    }

    pub fn get_left_n(&self) -> usize {
        self.queue.len() - self.get_played_n()
    }

    pub fn get_quote(&self) -> (String, String) {
        let i = self.get_played_n();
        get_quote(i + self.rng_seed)
    }
    pub fn get_played_games(&self) -> Vec<(&Match, i64)> {
        self.queue.clone().into_sorted_iter().filter_map(|(k, prior)|  {
            if prior < 0 {
                Some((self.match_list.get(&k).expect("Match list should include all queue elements"), prior))
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
        for (id, p) in &self.player_list {
            for round in 1..=(self.rounds) {
                if self.match_list.contains_key(&(p.id, player.id, round as u16)) {
                    return Err("Match Already exists".to_string());
                }
                if round % 2 == 1 {
                    let k = (*id, player.id, round as u16);
                    self.match_list.insert(
                        k, Match::new(p, &player, round as u16)
                    );
                    self.queue.push(k, 0);
                } else {
                    let k = (player.id, *id, round as u16);
                    self.match_list.insert(
                        k, Match::new(&player, p, round as u16)
                    );
                    self.queue.push(k, 0);
                }
            }
        }
        self.player_list.insert(player.id, player);
        self.update_priorities();
        Ok(())
    }

    pub fn add_result(&mut self, game_id: (u16, u16, u16), play1: Rpssl, play2: Rpssl) {
        debug!("Adding result to game {} {} {}", game_id.0, game_id.1, game_id.2);
        let m = self.match_list.get_mut(&game_id).unwrap();
        let player1 = self.player_list.get_mut(&m.player1).unwrap();
        let name1 = player1.name.clone();
        let result = play1.result(&play2);
        let score = result.get_score();
        let player1_score = play1.get_score() + score;
        let player2_score = play2.get_score() + (6 - score);

        player1.played += 1;
        player1.score += player1_score;
        debug!("Player {} has played {} games with a total score {}", player1.name, &player1.played, &player1.score);
        m.play1 = play1;
        m.play2 = play2;
        m.result = Some(result);
        let player2 = self.player_list.get_mut(&m.player2).unwrap();
        let name2 = player2.name.clone();
        player2.played += 1;
        player2.score += player2_score;
        debug!("Player {} has played {} games with a total score {}", player2.name, &player2.played, &player2.score);

        info!("Add result for game {} - {} (round {}), {} ({} points) - {} ({} points)", name1, name2, m.round, play1, player1_score, play2, player2_score);

        self.update_priorities();
    }

    pub fn remove_latest(&mut self) {
        info!("Remove latest play");
        let mut played_games = self.get_played_games();
        let (p1, p2, round) = if let Some((m,_p)) = played_games.iter_mut().last() {
            (m.player1, m.player2, m.round)
        } else {

            info!("No played games");
            return;
        };
        let _ = self.remove_result((p1,p2, round ));
        info!("Removed latest play {} {} {}", p1, p2, round);
        self.update_scores();
        self.update_priorities();
    }

    pub fn remove_result(&mut self, game_id: (u16, u16, u16)) -> Result<(), String> {
        let m = match self.match_list.get_mut(&game_id) {
            Some(m) => m,
            None => return Err("No such game".to_string()),
        };
        m.result = None;
        m.play1 = Rpssl::None;
        m.play2= Rpssl::None;
        info!("Removed play {} {}", game_id.0, game_id.1);
        Ok(())
    }

    pub fn update_priorities(&mut self) {
        debug!("Updating priorities");
        if self.player_list.is_empty() {
            info!("Player List is empty");
            return;
        }
        let played_games = self.player_list.values().map(|p| p.played as i64).sum::<i64>() / 2 + 1;
        let n_games = (self.rounds * (self.player_list.len() - 1)) as u16;

        for (k, m) in &self.match_list {
            debug!("Priority for game {} - {} (round {})", k.0, k.1, k.2);
            let player1 = match self.player_list.get(&m.player1) {
                Some(p) => p,
                None => {
                    info!("Error getting {}", m.player1);
                    panic!("");
                }
            };
            let player2 = self.player_list.get(&m.player2).unwrap();
            debug!("\t{} - {}", player1.name, player2.name);

            if m.play1.is_some() & m.play2.is_some() {

                match self.queue.get(k) {
                    Some((_i,p)) if p < &0 => {
                        debug!("Game already has negative priority");
                        continue;
                    },
                    _ => {
                        debug!("\tSet negative priority");
                        self.queue.change_priority(k, -played_games);
                        continue;
                    },
                }
            }
            let round = m.round;
            debug!("\tRound = {}", &round);
            let games_left1 = n_games - player1.played;
            let games_left2 = n_games - player2.played;
            debug!("\tGames left = {} / {}", &games_left1, &games_left2);
            let potential1 = (games_left1 * 9) as i64;
            let potential2 = (games_left2 * 9) as i64;
            debug!("\tPotential = {} / {}", &potential1, &potential2);
            let score1 = player1.score as i64;
            let score2 = player2.score as i64;
            debug!("\tScore = {} / {}", &score1, &score2);
            let priority = score1 - potential1 + score2 - potential2 + (round as i64) * 1000;
            debug!("\tPriority = {}", &priority);
            self.queue.change_priority(k, 99999 - priority);
        }
        debug!("priorities updated");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Match {
    pub player1: u16, 
    pub player2: u16,
    pub play1: Rpssl,
    pub play2: Rpssl,
    pub result: Option<RpsResult>,
    pub round: u16,
}

impl Match {
    fn new(player1: &Player, player2: &Player, round: u16) -> Self {
        Match {
            player1: player1.id,
            player2: player2.id,
            play1: Rpssl::None,
            play2: Rpssl::None,
            result: None,
            round,
        }
    }

    pub fn get_score(&self) -> (u16, u16) {
        if self.result.is_none() {
            return (0,0);
        }
        let score = self.result.as_ref().unwrap().get_score();
        let player1_score = self.play1.get_score() + score;
        let player2_score = self.play2.get_score() + (6 - score);
        (player1_score, player2_score)
    }

    fn get_score_for_player(&self, pid: u16) -> u16 {
        if self.result.is_none() {
            return 0;
        }
        if (pid != self.player1) & (pid != self.player2) {
            0
        } else {
            let (p1_score, p2_score) = self.get_score();
            if pid == self.player1 {
                p1_score
            } else {
                p2_score
            }
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

pub trait Playable: fmt::Display {
    fn str(&self) -> &str;
    fn get_score(&self) -> u16;
    fn new(inp: &str) -> Self;
    fn result(&self, other: &Self) -> RpsResult;
    fn is_none(&self) -> bool;
    fn is_some(&self) -> bool;
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, EnumIter)]
pub enum Rpssl {
    Rock,
    Paper,
    Scissors,
    Spock,
    Lizard,
    None,
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
    None,
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

impl Display for Rpssl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}


impl Playable for Rpssl {
    fn str(&self) -> &str {
        match self {
            Self::Rock => "🪨",
            Self::Paper => "📜",
            Self::Scissors => "✂️",
            Self::Lizard => "🦎",
            Self::Spock => "🖖",
            Self::None => "?",
        }
    }

    fn get_score(&self) -> u16 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
            Self::Lizard => 4,
            Self::Spock => 5,
            Self::None => 0,
        }
    }

    fn new(inp: &str) -> Self {
        match inp {
            "🪨" => Self::Rock,
            "📜" => Self::Paper,
            "✂️" => Self::Scissors,
            "🖖" => Self::Spock,
            "🦎" => Self::Lizard,
            "?" => Self::None,
            _ => panic!("Unknown string"),
        }
    }

    fn result(&self, other: &Rpssl) -> RpsResult {
        if self == other {
            RpsResult::Draw
        } else if other == &self.win().0 {
            RpsResult::Lose
        } else if other == &self.win().1 {
            RpsResult::Lose
        } else {
            RpsResult::Win
        }
    }

    fn is_none(&self) -> bool {
        self == &Rpssl::None
    }

    fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[allow(dead_code)]
impl Rpssl {
    fn win(&self) -> (Self, Self) {
        (FromPrimitive::from_u8((*self as u8 + 1) % 5).unwrap(),
        FromPrimitive::from_u8((*self as u8 + 3) % 5).unwrap())
    }

    fn lose(&self) -> (Self, Self) {
        (FromPrimitive::from_u8((*self as u8 + 4) % 5).unwrap(),
        FromPrimitive::from_u8((*self as u8 + 2) % 5).unwrap())
    }
}

impl Playable for Rps {
    fn get_score(&self) -> u16 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
            Self::None => 0,
        }
    }

    fn str(&self) -> &str {
        match self {
            Self::Rock => "🪨",
            Self::Paper => "📜",
            Self::Scissors => "✂️",
            Self::None => "?",
        }
    }

    fn new(inp: &str) -> Self {
        match inp {
            "🪨" => Self::Rock,
            "📜" => Self::Paper,
            "✂️" => Self::Scissors,
            _ => panic!("Unknown string"),
        }
    }

    fn result(&self, other: &Self) -> RpsResult {
        if self == other {
            RpsResult::Draw
        } else if other == &self.win() {
            RpsResult::Lose
        } else {
            RpsResult::Win
        }
    }

    fn is_none(&self) -> bool {
        self == &Self::None
    }

    fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[allow(dead_code)]
impl Rps {

    fn win(&self) -> Self {
        FromPrimitive::from_u8((*self as u8 + 1) % 3).unwrap()
    }

    fn lose(&self) -> Self {
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
    fn rpssl() {
        let rock = Rpssl::Rock;
        let paper = Rpssl::Paper;
        let scissors = Rpssl::Scissors;
        let lizard = Rpssl::Lizard;
        let spock = Rpssl::Spock;
        assert_eq!(rock.result(&rock), RpsResult::Draw);
        assert_eq!(rock.result(&paper), RpsResult::Lose);
        assert_eq!(rock.result(&scissors), RpsResult::Win);
        assert_eq!(lizard.result(&spock), RpsResult::Win);
        assert_eq!(lizard.result(&paper), RpsResult::Win);
        assert_eq!(rock.result(&lizard), RpsResult::Win);
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

        game.add_result((1, 2, 1), Rpssl::Rock, Rpssl::Scissors);
        game.add_result((1, 3, 1), Rpssl::Rock, Rpssl::Paper);

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
        let (g3, _p) = game.queue.pop().unwrap();
        assert_eq!(g3, (2,4 ,1));

        let scores = game.get_scores();
        assert_eq!(scores[0].id, 1);
        assert_eq!(scores[1].id, 3);
        assert_eq!(scores[2].id, 2);
        assert_eq!(scores[3].id, 4);
    }

    #[test]
    fn rounds() {
        let mut game = Game::new();
        let _ = game.add_player("Alice");
        let _ = game.add_player("Bob");
        let _ = game.add_player("Charlie");
        assert_eq!(game.get_left_n(), 3);
        game.set_rounds(2);
        dbg!(&game.match_list);
        assert_eq!(game.get_left_n(), 6);
    }

    #[test]
    fn game() {
        let mut game = Game::new();
        let _ = game.add_player("Alice");
        assert_eq!(game.player_list.len(), 1);
        assert_eq!(game.match_list.len(), 0);
        assert_eq!(game.queue.len(), 0);
        let _ = game.add_player("Bob");
        assert_eq!(game.player_list.len(), 2);
        assert_eq!(game.match_list.len(), 1);
        assert_eq!(game.queue.len(), 1);

        let _ = game.add_player("Charlie");
        assert_eq!(game.player_list.len(), 3);
        assert_eq!(game.match_list.len(), 3);
        assert_eq!(game.queue.len(), 3);

        //game.update_priorities();

        game.add_result((1,2, 1), Rpssl::Rock, Rpssl::Scissors);
        let p1 = game.player_list.get(&1).unwrap();
        let p2 = game.player_list.get(&2).unwrap();
        assert_eq!(p1.score, 7);
        assert_eq!(p2.score, 3);

        assert_eq!(p1.played, 1);
        assert_eq!(p2.played, 1);
        //game.update_priorities();
        dbg!(&game.queue);
        dbg!(&game.player_list);
        let (g3, _p) = game.queue.pop().unwrap();
        // 2,3 should come first since player 1 has a higher score
        // Thus 1,3 has lower priority than 2,3 
        assert_eq!(g3, (2, 3, 1));
        let (g3, _p) = game.queue.pop().unwrap();
        assert_eq!(g3, (1, 3, 1));

        {
            let scores = game.get_scores();
            //dbg!(&game.queue);
            assert_eq!(scores[0].id, 1);
            assert_eq!(scores[1].id, 2);
            assert_eq!(scores[2].id, 3);
        }
        {
            let played_games = game.get_played_games();
            dbg!(&played_games);
            assert_eq!(played_games.len(), 1);
        }

        // #3 wins, gets 6 + 2 points, 1 point for #1
        game.add_result((1,3, 1), Rpssl::Rock, Rpssl::Paper);
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
