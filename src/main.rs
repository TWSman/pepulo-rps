use leptos::{ev::SubmitEvent, ev::MouseEvent, *};
use leptos::html::Input;
use Jan2024::Game;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

/// Shows progress toward a goal
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default=100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
        max=max
        value=progress
        />
    }
}

//#[component]
//pub fn PlayerList(
//    game: Signal<Game>
//) -> impl IntoView {
//    let (g, _set_g) = game;
//    view! {
//        <For
//            each=move || g.get().player_list
//            key=|game| g.key.clone()
//            let:child
//        >
//            <p>{child.name}</p>
//        </For>
//    }
//}

#[component]
fn Scores() -> impl IntoView {
    let (show_scoring, set_scoring) = create_signal(false);
    view! {
        <div class="nnn" id="scores" on:click=move |_| set_scoring.update(|value| *value = true)>
            <Show
            when=move || {show_scoring.get() }
            fallback=|| view! { <h1>"Säännöt"</h1>}
            >
            <p on:click=move |_| set_scoring.update(|value| *value = false)>X</p>
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
            </ul>
    </Show>
    </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (game, set_game) = create_signal(Game::new());
    set_game.update(|g| g.add_player("Alice"));
    set_game.update(|g| g.add_player("Bob"));
    let (show_names, set_names) = create_signal(false);
    let (show_games, set_games) = create_signal(false);

    view! {
        <div id="container">
        <Scores/>
        <div class="nnn" id="player_list" on:click=move |_| set_names.update(|value| *value = true)>
        <Show
            when=move || {show_names.get() }
            fallback=|| view! { <h1>"Tilanne"</h1>}
        >
            <p on:click=move |_| set_names.update(|value| *value = false)>X</p>
            <table>
                <head>
                <tr>
                    <th>Name</th>
                    <th>Score</th>
                </tr>
                </head>
                <tr> <td>Alice</td> <td>0</td> </tr>
                <tr> <td>Bob</td> <td>10</td> </tr>
            </table>
        </Show>
        </div>

        <div class="nnn" id="games" on:click=move |_| set_games.update(|value| *value = true)>
        <Show
            when=move || {show_games.get() }
            fallback=|| view! { <h1>"Pelaamaan"</h1>}
        >
            <p on:click=move |_| set_games.update(|value| *value = false)>X</p>
            PlaceHolder
        </Show>
        </div>
        </div>
    }
}
