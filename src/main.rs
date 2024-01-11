use leptos::{ev::SubmitEvent, *};
use leptos::html::Input;
use console_log;
use Jan2024::{Game,Rps, Player};
use log::Level;
use log::info;

// TODO: Reaktiivisuus
// Pelien lisääminen bugaa

fn main() {
    _ = console_log::init_with_level(Level::Debug);
    //let game = Game::new();
    //let app_state = Arc::new(Mutex::new(game));
    info!("It works!");
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
    let (name, _set_name) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();
        // Get value from input
        let value = input_element.get().expect("<input> to exist").value();
        game.update(|g| {
            match g.add_player(&value) {
                Ok(()) => (),
                Err(_) => info!("Player {} already exists", value),
            }
        });
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Lisää"/>
        </form>
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
            PlayerScore {name:i.name.clone(), played:i.played.clone(), score:i.score.clone()}
        })
        .collect::<Vec<_>>();
    view! {
        <table>
            <tr>
                <th>Pelaaja</th>
                <th>Pelit</th>
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
fn Scores() -> impl IntoView {
    let (show_scoring, set_scoring) = create_signal(false);
    view! {
        <div class="nnn" id="scores" on:click=move |_| set_scoring.update(|value| *value = true)>
            <Show when=move || { show_scoring.get() } fallback=|| view! { <h1>"Säännöt"</h1> }>
                <p class="close" on:click=move |_| set_scoring.update(|value| *value = false)>
                    X
                </p>
                <ul>
                    <li>"KPS - kaikki vastaan kaikki"</li>
                    <li>"Yksinkertainen sarja"</li>
                    <li>
                        "Pisteitä saa tuloksesta" <ul>
                            <li>" Voitto: 6 pistettä"</li>
                            <li>" Tasapeli: 3 pistettä"</li>
                            <li>" Tappio: 0 pistettä"</li>
                        </ul>
                    </li>
                    <li>
                        "ja pelatusta kädestä" <ul>
                            <li>" Sakset: 3 pistettä"</li>
                            <li>" Paperi: 2 pistettä"</li>
                            <li>" Kivi: 1 piste"</li>
                        </ul>
                    </li>
                    <li>Tasapisteissä voittajan ratkaisee tavallinen, paras viidestä - kaksinkamppailu</li>
                </ul>
            </Show>
        </div>
    }
}



#[component]
pub fn NextMatch(
    #[prop(into)]
    game: WriteSignal<Game>,
    player1: Player,
    player2: Player,
) -> impl IntoView {
    //let (value, set_value) = create_signal("B".to_string());
    //

    let (value, set_value) = create_signal("None".to_string());
    let (value2, set_value2) = create_signal("None".to_string());
    let player1_name = player1.name.to_string();
    let player2_name = player2.name.to_string();

    let player1_id = player1.id;
    let player2_id = player2.id;

    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();
        let play1 = value.get();
        let play2 = value2.get();

        if (play1 == "None") | (play2 == "None") {
            info!("One option was None");
            return;
        }
        //let play1 = Rps::new(play1);
        let play1 = Rps::new(&play1[..]);
        let play2 = Rps::new(&play2[..]);
        let message = format!("Add result for {} {} v. {} {}", player1_id, play1, player2_id, play2).to_string();
        info!("{}", message);
        game.update(|g| {
            info!("message = {message}");
            g.add_result((player1_id,player2_id), play1, play2)}
        );
    };
    view! {
        <li>
            <form on:submit=on_submit>
                <p>
                    {player1_name} "   " // <span class="play_select">"🪨"</span>
                    // <span class="play_select">"📜"</span>
                    // <span class="play_select">"✂️"</span>

                    <select on:change=move |ev| {
                        let new_value = event_target_value(&ev);
                        set_value.set(new_value);
                    }>
                        <SelectOption value is="None"/>
                        <SelectOption value is=Rps::Rock.str()/>
                        <SelectOption value is=Rps::Paper.str()/>
                        <SelectOption value is=Rps::Scissors.str()/>
                    </select>
                    " Vs. " {player2_name} "   "
                    <select on:change=move |ev| {
                        let new_value = event_target_value(&ev);
                        set_value2.set(new_value);
                    }>
                        <SelectOption value=value2 is="None"/>
                        <SelectOption value=value2 is=Rps::Rock.str()/>
                        <SelectOption value=value2 is=Rps::Paper.str()/>
                        <SelectOption value=value2 is=Rps::Scissors.str()/>
                    </select>
                </p>
                <input type="submit" value="Lisää"/>
            </form>
        </li>
        // <input type="text"
        // value=name
        // node_ref=input_element
        ///>
        "hei"
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

#[derive(Debug, Clone)]
struct GameScore {
    name1: String,
    name2: String,
    play1: String,
    play2: String,
    prior: i64,
}

#[component]
pub fn MatchList(
game: ReadSignal<Game>,
set_game: WriteSignal<Game>,
) -> impl IntoView {
    let data = move || game.get().get_played_games()
        .iter()
        .map(|(m,p)| {

            let p1 = m.player1;
            let p2 = m.player2;
            let player1 = game.get().get_player_name(p1);
            let player2 = game.get().get_player_name(p2);
            let play1 = m.play1.unwrap().str().to_string();
            let play2 = m.play2.unwrap().str().to_string();

            GameScore {
                name1: player1.clone(),
                name2: player2.clone(),
                play1: play1.clone(),
                play2: play2.clone(),
                prior: *p,
            }
        })
        .collect::<Vec<_>>();


    view! {
        <table>
            <tr>
                <th>Prioriteetti</th>
                <th>Pelaaja 1</th>
                <th></th>
                <th>Pelaaja 2</th>
            </tr>
            <For
                each=move || { data() }
                key=|p| (p.name1.clone(), p.name2.clone())
                children=|child| {
                    view! {
                        <tr>
                            <td>{child.prior}</td>
                            <td>{child.name1} " - " {child.play1}</td>
                            <td>"Vs."</td>
                            <td>{child.name2} " - " {child.play2}</td>
                        </tr>
                    }
                }
            />

        </table>
        <h2>Seuraavana</h2>

        <ul>
            {game
                .with(|data| {
                    match data.get_next_game() {
                        Some(m) => {
                            let p1 = m.player1;
                            let p2 = m.player2;
                            let player1 = data.get_player(p1);
                            let player2 = data.get_player(p2);
                            view! { <NextMatch game=set_game player1=player1 player2=player2/> }
                                .into_view()
                        }
                        None => view! { <p>"No more games"</p> }.into_view(),
                    }
                })}

        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    let (game, set_game) = create_signal(Game::new());
    //set_game.update(|g| {let _ = g.add_player("Alice");});
    //set_game.update(|g| {let _ = g.add_player("Bob");});
    //set_game.update(|g| {let _ = g.add_player("Charlie");});
    //set_game.update(|g| {let _ = g.add_player("Daniel");});
    //set_game.update(|g| g.add_result((1,2), Rps::Rock, Rps::Scissors));
    //set_game.update(|g| g.add_result((3,4), Rps::Rock, Rps::Paper));
    let (show_names, set_names) = create_signal(false);
    let (show_games, set_games) = create_signal(false);

    let quote = move || {
        let (quote, author) = game.get().get_quote();
        view! {<p>"\"" {quote} "\""</p><p>" - "{author}</p>}
    };

    view! {
        <div id="container">
            <div class="header" id="header">
                <h1>"PePuLo KPS Liiga"</h1>
            </div>
            <Scores/>
            <div
                class="nnn"
                id="player_list"
                on:click=move |_| set_names.update(|value| *value = true)
            >
                <Show when=move || { show_names.get() } fallback=|| view! { <h1>"Pisteet"</h1> }>
                    <p class="close" on:click=move |_| set_names.update(|value| *value = false)>
                        X
                    </p>
                    <PlayerList game=game/>
                    // <NameInput game=set_game/>
                    <NameInput game=set_game/>

                </Show>
            </div>
            <div class="nnn" id="games" on:click=move |_| set_games.update(|value| *value = true)>
                <Show when=move || { show_games.get() } fallback=|| view! { <h1>"Pelaamaan"</h1> }>
                    <p class="close" on:click=move |_| set_games.update(|value| *value = false)>
                        X
                    </p>
                    <MatchList game=game set_game=set_game/>
                </Show>
            </div>
            <div class="nnn" id="quote">
                {quote}

            </div>
        </div>
    }
}
