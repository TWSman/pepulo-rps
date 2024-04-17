use leptos::{ev::SubmitEvent, *};
use leptos::html::Input;
use pepulo_rps::{Game,Rpssl,Playable,GameMode};
use log::Level;
use log::info;
use log::debug;
use itertools::Itertools;

fn main() {
    _ = console_log::init_with_level(Level::Info);
    leptos::mount_to_body(|| view! { <App/> })
}


#[server(Logging, "/logging")]
pub async fn add_log(message: String) -> Result<(), ServerFnError> {
    //let mut conn = db().await?;
    prinln!(&message);
    Ok(())
}

#[derive(Debug, Clone)]
struct PlayerScore {
    name: String,
    played: u16,
    score: u16,
}

#[component]
pub fn NameInput(
    #[prop(into)]
    game: WriteSignal<Game>
) -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();
        // Get value from input
        let value = input_element.get().expect("<input> to exist").value();
        set_name.update(|n| *n = "".to_string());
        game.update(|g| {
            match g.add_player(&value) {
                Ok(()) => (),
                Err(_) => info!("Player {} already exists", value),
            }
        });
    };
    view! {
        <div id="new_player">
        <form on:submit=on_submit>
            //<label for="newp">"Uusi pelaaja "</label>
            <input type="text" id="newp" value=name prop:value=name node_ref=input_element/>
            <input type="submit" value="Lisää"/>
        </form>
        </div>
    }
}

#[component]
pub fn PlayerList(
    game: ReadSignal<Game>,
) -> impl IntoView {

    //let input_element: NodeRef<Input> = create_node_ref();
    let data = move || game.get().get_scores()
        .iter()
        .map(|i| {
            PlayerScore {name:i.name.clone(), played:i.played, score:i.score}
        })
        .collect::<Vec<_>>();
    view! {
        <table>
            <tr>
                <th>Pelaaja</th>
                <th>Ottelut</th>
                <th>Pisteet</th>
            </tr>
            <For
                each=move || { data() }
                key=|p| (p.name.clone(), p.played, p.score)
                children=|child| {
                    view! {
                        <tr>
                            <td>{child.name}</td>
                            <td>{child.played}</td>
                            <td>{child.score}</td>
                        </tr>
                    }
                }
            />

        </table>
    }
}

#[component]
fn Scores(
    #[prop(into)]
    game: ReadSignal<Game>,
) -> impl IntoView {
    let (show_scoring, set_scoring) = create_signal(false);
    view! {
        <div class="nnn" id="scores" on:click=move |_| set_scoring.update(|value| *value = true)>
            <Show when=move || { show_scoring.get() } fallback=|| view! { <h1>"Säännöt"</h1>}>
                <p class="close" on:click=move |_| set_scoring.update(|value| *value = false)>
                    X
                </p>
                <ul>
                {
                    if game.get().get_mode().str() == "RPS" {
                        view!{
                            <li>"KPS - kaikki vastaan kaikki"</li>
                        }
                    }else {
                        view!{
                            <li>"KPSLV - kaikki vastaan kaikki"</li>
                        }
                    }
                }
                    <li>"Yksi peli kerrallaan"</li>
                    <li>"Kaksinkertainen sarja"</li>
                    <li>
                        "Pisteitä saa tuloksesta" <ul>
                            <li>" Voitto: 6 pistettä"</li>
                            <li>" Tasapeli: 3 pistettä"</li>
                            <li>" Tappio: 0 pistettä"</li>
                        </ul>
                    </li>
                {if game.get().get_mode().str() == "RPS" {
                    view!{
                        <li>
                            "ja pelatusta kädestä (riippumatta tuloksesta)" <ul>
                                <li>" Sakset: 3 pistettä"</li>
                                <li>" Paperi: 2 pistettä"</li>
                                <li>" Kivi: 1 piste"</li>
                            </ul>
                        </li>
                        }
                    } else {
                    view! {
                        <li>
                            "ja pelatusta kädestä (riippumatta tuloksesta)" <ul>
                                <li>" Vampyyri: 5 pistettä"</li>
                                <li>" Lisko: 4 pistettä"</li>
                                <li>" Sakset: 3 pistettä"</li>
                                <li>" Paperi: 2 pistettä"</li>
                                <li>" Kivi: 1 piste"</li>
                            </ul>
                        </li>
                        }
                    }
            }
                    <li>Eniten pisteitä kerännyt on voittaja</li>
                    <li>Tasapisteissä voittajan ratkaisee tavallinen, paras viidestä - kaksinkamppailu</li>
                    <li>"Psyykkinen sodankäynti on sallittua"</li>
                </ul>
            </Show>
        </div>
    }
}


#[component]
pub fn CurrentMatch(
    #[prop(into)]
    game: ReadSignal<Game>,
    set_game: WriteSignal<Game>,
) -> impl IntoView {
    //let (value, set_value) = create_signal("B".to_string());
    //

    let (value, set_value) = create_signal("?".to_string());
    let (value2, set_value2) = create_signal("?".to_string());
        
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();
        let play1 = value.get();
        let play2 = value2.get();

        if (play1 == "?") | (play2 == "?") {
            info!("One option was ?");
            return;
        }
        set_value.update(|v| *v = "?".to_string());
        set_value2.update(|v| *v = "?".to_string());
        let m = game.get().get_next_game().unwrap().clone();
        let player1_id = m.player1;
        let player2_id = m.player2;
        let round = m.round;
        let play1 = Rpssl::new(&play1[..]);
        let play2 = Rpssl::new(&play2[..]);
        let message = format!("Add result for {} {} v. {} {}", player1_id, play1, player2_id, play2).to_string();
        debug!("{}", message);
        set_game.update(|g| {
            //spawn_local(async {
            //    add_log("So much to do!".to_string()).await;
            //});
            g.add_result((player1_id,player2_id, round), play1, play2)}
        );
    };

    let mode = move || game.with(|g| g.get_mode());

    move || match game.get().get_next_game() {
        Some(m) => {
            let p1 = m.player1;
            let p2 = m.player2;
            let player1_name = game.get().get_player_name(p1).unwrap();
            let player2_name = game.get().get_player_name(p2).unwrap();
            let n_games = game.get().get_left_n();

            view! {
                <form on:submit=on_submit>
                    // <p>"Round" {round}</p>
                    <p id="seuraavana">
                        {player1_name} "   " // <span class="play_select">"🪨"</span>
                        // <span class="play_select">"📜"</span>
                        // <span class="play_select">"✂️"</span>
                        <select id="player1select" on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_value.set(new_value);
                        }>
                            <SelectOption value is="?"/>
                            <SelectOption value is=Rpssl::Rock.str()/>
                            <SelectOption value is=Rpssl::Paper.str()/>
                            <SelectOption value is=Rpssl::Scissors.str()/>
                            { if mode() == GameMode::RPSSL {
                                view! {
                                    <SelectOption value is=Rpssl::Vampire.str()/>
                                    <SelectOption value is=Rpssl::Lizard.str()/>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }
                            }
                        </select>
                        " Vs. " {player2_name} "   "
                        <select id="player2select" on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_value2.set(new_value);
                        }>
                            <SelectOption value=value2 is="?"/>
                            <SelectOption value=value2 is=Rpssl::Rock.str()/>
                            <SelectOption value=value2 is=Rpssl::Paper.str()/>
                            <SelectOption value=value2 is=Rpssl::Scissors.str()/>
                            { if mode() == GameMode::RPSSL {
                                view! {
                                    <SelectOption value=value2 is=Rpssl::Vampire.str()/>
                                    <SelectOption value=value2 is=Rpssl::Lizard.str()/>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }
                            }
                        </select>
                        <input type="submit" value="Lisää"/>
                    </p>
                </form>
                    {if n_games > 1 {
                        view! {
                            <p>{n_games} " peliä jäljellä"</p>
                        }
                    } else {
                            view! {
                                <p>"1 peli jäljellä"</p>
                            }
                        }
                    }
            }.into_view()
        }
        _ => view! {<p>"-"</p>}.into_view(),
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get() == is>
            {is}
        </option>
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct GameScore {
    name1: String,
    name2: String,
    play1: String,
    play2: String,
    id1: u16,
    id2: u16,
    score1: u16,
    score2: u16,
    prior: i64,
    round: u16,
}

#[component]
pub fn MatchList(
game: ReadSignal<Game>,
set_game: WriteSignal<Game>,
) -> impl IntoView {

    let data = move || game.get().get_played_games()
        .iter()
        .rev()
        .map(|(m,p)| {

            let id1 = m.player1;
            let id2 = m.player2;
            let player1 = game.get().get_player_name(id1).unwrap();
            let player2 = game.get().get_player_name(id2).unwrap();
            let (score1, score2) = m.get_score();
            let play1 = m.play1.str().to_string();
            let play2 = m.play2.str().to_string();

            GameScore {
                name1: player1.clone(),
                name2: player2.clone(),
                play1,
                play2,
                id1,
                id2,
                score1,
                score2,
                prior: *p,
                round: m.round,
            }
        }).with_position()
        .collect::<Vec<_>>();

    let quote = move || {
        let (quote, author) = game.get().get_quote();
        view! {<p>"\"" {quote} "\""</p><p>" - "{author}</p>}
    };

    view! {
        <h2>Seuraavana:</h2>
        <p>
            <CurrentMatch game=game set_game=set_game/>
        </p>
        <hr/>
        <div>{quote}</div>
        <hr/>

        <table>
            <tr>
                <td class="trashcan"
            on:click = move |_| {set_game.update(|g| g.remove_latest())}
            >"🗑️"</td>
            </tr>
            <For
                each=move || { data() }
                key=|(_pos, p)| (p.name1.clone(), p.name2.clone(), p.round)
                children=|(_pos, child)| {
                    view! {
                        <tr>
                            //<td>{child.round}</td>
                            //<td>{child.prior}</td>
                            <td>{child.play1}</td>
                            <td>{child.score1} "p"</td>
                            <td style="text-align:right;">{child.name1} </td>
                            <td>"Vs."</td>
                            <td>{child.name2}</td>
                            <td>{child.play2}</td>
                            <td>{child.score2} "p"</td>
                            
                        </tr>
                    }
                }
            />

        </table>

    }
}

#[component]
fn App() -> impl IntoView {
    let (game, set_game) = create_signal(Game::new());
    //set_game.update(|g| {let _ = g.set_mode(GameMode::RPSSL);});
    let (show_names, set_names) = create_signal(false);
    let (show_games, set_games) = create_signal(false);
    let (show_options, set_options) = create_signal(false);

    view! {
        <div class="header" id="header">
            <h1>"Kivi-Paperi-Sakset-Lisko-Vampyyri"</h1>
        </div>
        <div id="container">
            <Scores game=game/>
            <div class="nnn" id="games" on:click=move |_| set_games.update(|value| *value = true)>
                <Show when=move || { show_games.get() } fallback=|| view! { <h1>"Pelaamaan"</h1> }>
                    <p class="close" on:click=move |_| set_games.update(|value| *value = false)>
                        X
                    </p>
                    <MatchList game=game set_game=set_game/>
                </Show>
            </div>
            <div
                class="nnn"
                id="player_list"
                on:click=move |_| set_names.update(|value| *value = true) >
                <Show when=move || { show_names.get() } fallback=|| view! { <h1>"Pelaajat"</h1> }>
                    <p class="close" on:click=move |_| set_names.update(|value| *value = false)>
                        X
                    </p>
                    <PlayerList game=game/>
                    // <NameInput game=set_game/>
                    <NameInput game=set_game/>

                </Show>
            </div>
            <div class="nnn" id="options" on:click=move |_| set_options.update(|value| *value = true)>
                <Show when=move || { show_options.get() } fallback=|| view! { <h1>"Asetukset"</h1> }>
                    <p class="close" on:click=move |_| set_options.update(|value| *value = false)>
                        X
                    </p>
                    <Setup game=game set_game=set_game/>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn Setup(
game: ReadSignal<Game>,
set_game: WriteSignal<Game>,
) -> impl IntoView {
    let current_mode = move || match game.get().get_mode().str() {
        "RPS" => "KPS".to_string(),
        "RPSSL" => "KPSLV".to_string(),
        _ => "Unknown".to_string()
    };
    let rounds = move || game.get().get_rounds();
    let debug = move || {
        set_game.update(|g| g.set_rounds(2));
        set_game.update(|g| {let _ = g.add_player("Alice");});
        set_game.update(|g| {let _ = g.add_player("Bob");});
        set_game.update(|g| {let _ = g.add_player("Charlie");});
        set_game.update(|g| {let _ = g.add_player("Daniel");});
        set_game.update(|g| {let _ = g.add_player("Eric");});
        set_game.update(|g| g.add_result((1,2,1), Rpssl::Rock, Rpssl::Scissors));
        set_game.update(|g| g.add_result((3,4,1), Rpssl::Rock, Rpssl::Paper));
    };
    view! {
        <h2>Asetukset:</h2>
        <button on:click=move |_| set_game.update(|game| { let _ =game.set_mode(GameMode::RPS);})>KPS</button>
        <button on:click=move |_| set_game.update(|game| { let _ =game.set_mode(GameMode::RPSSL);})>KPSLV</button>
        <p>"Peli: " {current_mode}</p>
        <p>"Kierroksia: " {rounds} " "
            <button on:click=move |_| set_game.update(|game| { let _ =game.add_rounds();})>+</button>
            <button on:click=move |_| set_game.update(|game| { let _ =game.remove_rounds();})>-</button>
            </p>
        <p>
            <button on:click=move |_| debug()>Debug</button>
        </p>
        <p>
            <button on:click=move |_| set_game.update(|game| {let _ = game.empty();})>Empty</button>
        </p>
    }
}
