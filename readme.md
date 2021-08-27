# A Rust client for steamgriddb.com

This project is a wrapper for the steamgriddb api, written in Rust.

## Getting started

The easiest way to get started is using the Client.

```rust
use steamgriddb_api::client::Client;
use steamgriddb_api::query_parameters::QueryType::Grid;
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("my_auth_key");
    let games = client.search("Celeste").await?;
    let first_game = games.iter().next().unwrap();
    assert_eq!("Celeste", first_game.name);
    let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;
    Ok(())
 }
```

The client calls the API using the [reqwest](https://crates.io/crates/reqwest) crate and parses the results using the [serde](https://crates.io/crates/serde) crate.

```rust
use steamgriddb_api::images::*;
use steamgriddb_api::query_parameters::*;
let url = get_images_by_platform_ids_url("https://www.steamgriddb.com/api/v2", &Platform::Steam, &["107500", "107510"], &QueryType::Grid(None));
```


### Use an Auth Key

In the examples above the client is given a ``"my_auth_key"``, you need to substitute this string with your own key. 
You can get a key here: 
https://www.steamgriddb.com/profile/preferences/api


## Progress

This crate currently only supports getting data from the steamgriddb api. Uploading will come in a later version.