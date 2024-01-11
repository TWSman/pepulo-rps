use leptos::{ev::SubmitEvent, ev::MouseEvent, *};
use leptos::html::Input;
use console_log;
use Jan2024::{Game,Rps, Player};
use itertools::{Itertools, Position};
use log::Level;
use log::{info, debug};

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

#[component]
pub fn PlayerList(
    #[prop(into)]
    game: ReadSignal<Game>
) -> impl IntoView {
    view! {
        <table>
            <tr>
                <th>Pelaaja</th>
                <th>Pelit</th>
                <th>Pisteet</th>
            </tr>
        {game.with(|data| data.get_scores().into_iter().map(|p| {
            view! {
                <tr>
                    <td>{p.name.clone()}</td>
                    <td>{p.played}</td>
                    <td>{p.score}</td>
                </tr>
            }
        }).collect_view()
        )}
        </table>
    }
}

#[component]
fn Scores() -> impl IntoView {
    let (show_scoring, set_scoring) = create_signal(false);
    view! {
        <div class="nnn" id="scores" on:click=move |_| set_scoring.update(|value| *value = true)>
            <Show
            when=move || {show_scoring.get() }
            fallback=|| view! { <h1>"Säännöt"</h1>}
            >
            <p class="close" on:click=move |_| set_scoring.update(|value| *value = false)>X</p>
            <ul>
            <li>"RPS - kaikki vastaan kaikki"</li>
            <li>"Yksinkertainen sarja"</li>
            <li>"Pisteitä tuloksesta"
            <ul>
            <li>" Voitto: 6 pistettä"</li>
            <li>" Tasapeli: 3 pistettä"</li>
            <li>" Tappio: 0 pistettä"</li>
            </ul>
            </li>
            <li>"Ja pelatusta"
            <ul>
            <li>" Sakset: 3 pistettä"</li>
            <li>" Paperi: 2 pistettä"</li>
            <li>" Kivi: 1 piste"</li>
            </ul>
            </li>
            <li> Tasapisteissä voittajan 
            </li>
            </ul>
    </Show>
    </div>
    }
}


#[component]
pub fn NameInput(
    #[prop(into)]
    game: WriteSignal<Game>
) -> impl IntoView {
    let (name, set_name) = create_signal("Uusi pelaaja".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();
        // Get value from input
        let value = input_element.get().expect("<input> to exist").value();
        game.update(|g| g.add_player(&value));
        //set_name.set(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Lisää"/>
        </form>
    //<p>"Name is: " {name}</p>
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
            return;
        }
        let message = format!("Add result for {} v. {}", player1_id, player2_id).to_string();
        info!("{}", message);
        game.update(|g| {
            //spawn_local(async {
            //    let _ = add_log(message).await;
            //});
            dbg!("message = {message}");
            g.add_result((player1_id,player2_id), Rps::Rock, Rps::Paper)}
        );
    };
    view! {
        <li>
        <form on:submit=on_submit>
            <p>{player1_name} "   "
            //<span class="play_select">"🪨"</span>
            //<span class="play_select">"📜"</span>
            //<span class="play_select">"✂️"</span>

            <select on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value.set(new_value);
            }>
                <SelectOption value is="None"/>
                <SelectOption value is="🪨"/>
                <SelectOption value is="📜"/>
                <SelectOption value is="✂️"/>
            </select>

            " Vs. "
            {player2_name} "   "
            <select on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value2.set(new_value);
            }>
                <SelectOption value is="None"/>
                <SelectOption value is="🪨"/>
                <SelectOption value is="📜"/>
                <SelectOption value is="✂️"/>
            </select>
            </p>
            <input type="submit" value="Lisää"/>
        </form>
        </li>
            //<input type="text"
                //value=name
                //node_ref=input_element
            ///>
        "hei"
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value.get() == is
        >
            {is}
        </option>
    }
}

#[component]
pub fn MatchList(
#[prop(into)]
game: ReadSignal<Game>,
#[prop(into)]
set_game: WriteSignal<Game>,
) -> impl IntoView {
view! {
    <table>
        {game.with(|data| data.get_played_games().into_iter().map(|m| {
            let p1 = m.player1;
            let p2 = m.player2;
            let player1 = data.get_player_name(p1);
            let player2 = data.get_player_name(p2);
            let play1 = m.play1.unwrap().str().to_string();
            let play2 = m.play2.unwrap().str().to_string();
            view! {
                <tr>
                    <td>{player1} " - " { play1 }</td>

                    <td>"Vs."</td>
                    <td>{player2} " - " {play2}</td>
                </tr>
            }
        }).collect_view()
        )}
        </table>
        <h2>Seuraavana</h2>

        <ul>
        {game.with(|data| data.get_next_games(1).into_iter().with_position().map(|(pos,m)| {
            match pos {
                Position::First | Position::Only => {
                    let p1 = m.player1;
                    let p2 = m.player2;
                    let player1 = data.get_player(p1);
                    let player2 = data.get_player(p2);
                        view! {<NextMatch game=set_game player1=player1 player2=player2/>}.into_view()
                },
                _ => {
                    let p1 = m.player1;
                    let p2 = m.player2;
                    //let play1 = m.play1.unwrap().str().to_string();
                    //let play2 = m.play2.unwrap().str().to_string();
                    let player1 = data.get_player_name(p1);
                    let player2 = data.get_player_name(p2);
                    view! {
                        <li>
                            <p>{player1}
                            " Vs. "
                            {player2}</p>
                        </li>
                }.into_view()
            }
        }
        }).collect_view()
        )}
        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    let (game, set_game) = create_signal(Game::new());
    set_game.update(|g| g.add_player("Alice"));
    set_game.update(|g| g.add_player("Bob"));
    set_game.update(|g| g.add_player("Charlie"));
    set_game.update(|g| g.add_player("Daniel"));
    set_game.update(|g| g.add_result((1,2), Rps::Rock, Rps::Scissors));
    set_game.update(|g| g.add_result((3,4), Rps::Rock, Rps::Paper));
    let (show_names, set_names) = create_signal(false);
    let (show_games, set_games) = create_signal(false);

    view! {
        <div id="container">
        <Scores/>
        <div class="nnn" id="player_list" on:click=move |_| set_names.update(|value| *value = true)>
        <Show
            when=move || {show_names.get() }
            fallback=|| view! { <h1>"Pisteet"</h1>}
        >
            <p class="close" on:click=move |_| set_names.update(|value| *value = false)>X</p>
            <PlayerList game=game/>
            <NameInput game=set_game/>
        </Show>
        </div>
        <div class="nnn" id="games" on:click=move |_| set_games.update(|value| *value = true)>
        <Show
            when=move || {show_games.get() }
            fallback=|| view! { <h1>"Pelaamaan"</h1>}
        >
            <p class="close" on:click=move |_| set_games.update(|value| *value = false)>X</p>
            <MatchList game=game set_game=set_game/>
        </Show>
        </div>
        </div>
    }
}
