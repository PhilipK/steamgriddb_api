#[derive(Debug, Clone)]

/// Urls for steam static files
pub struct SteamStaticUrls {
    /// Steam header url
    pub header: String,
    /// Steam capsule url
    pub capsule: String,    
    /// Steam hero url
    pub hero: String,
    /// Steam logo url
    pub logo: String,
}

impl SteamStaticUrls {

    /// Create a new instance of SteamStaticUrls
    pub fn new(steam_id: &str) -> Self {
        Self{
            header:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/header.jpg", steam_id=steam_id),
            capsule:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/library_600x900_2x.jpg", steam_id=steam_id),
            hero:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/library_hero.jpg", steam_id=steam_id),
            logo:format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{steam_id}/logo.png", steam_id=steam_id),
        }
    }
}
