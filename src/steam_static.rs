#[derive(Debug, Clone)]
pub struct SteamStaticUrls {
    pub header: String,
    pub capsule: String,
    pub hero: String,
    pub logo: String,
}

impl SteamStaticUrls {
    pub fn new(steam_id: &str) -> Self {
        Self{
            header:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/header.jpg", steam_id=steam_id),
            capsule:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/library_600x900_2x.jpg", steam_id=steam_id),
            hero:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/library_hero.jpg", steam_id=steam_id),
            logo:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/logo.png", steam_id=steam_id),
        }
    }
}
