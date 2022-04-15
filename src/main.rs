use rspotify::{
    clients::{BaseClient, OAuthClient},
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

fn main() {
    // Initialise the client
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    spotify.refetch_token();

    let stream = spotify.current_user_saved_tracks(None);
    println!("Items:");
    for item in stream {
        println!("* {}", item.unwrap().track.name);
    }

    // // Get a list of the users playlists
    // let playlists = spotify.current_user_playlists();
    // println!("Playlists:");
    // for playlist in playlists {
    //     println!("  * {}", playlist.unwrap().name);
    // }
}
