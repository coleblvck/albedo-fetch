use reqwest::{Client, Error};
use serde::Deserialize;

const SOL_BODIES_URL: &str = "https://api.le-systeme-solaire.net/rest/bodies";
const _SOL_BODIES_COUNT_URL: &str = " https://api.le-systeme-solaire.net/rest/knowncount";

#[derive(Deserialize)]
pub struct SolBodies {
    pub bodies: Vec<SolBody>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolBody {
    pub id: String,
    pub name: String,
    pub english_name: String,
    pub is_planet: bool,
    pub moons: Option<Vec<SolBodyMoon>>,
    pub semimajor_axis: u128,
    pub perihelion: u128,
    pub aphelion: u128,
    pub eccentricity: f64,
    pub inclination: f64,
    pub mass: Option<SolBodyMass>,
    pub vol: Option<SolBodyVol>,
    pub density: f64,
    pub gravity: f64,
    pub escape: f64,
    pub mean_radius: f64,
    pub equa_radius: f64,
    pub polar_radius: f64,
    pub flattening: f64,
    pub dimension: String,
    pub sideral_orbit: f64,
    pub sideral_rotation: f64,
    pub around_planet: Option<SolBodyAroundPlanet>,
    pub discovered_by: String,
    pub discovery_date: String,
    pub alternative_name: String,
    pub axial_tilt: f64,
    pub avg_temp: u128,
    pub main_anomaly: f64,
    pub arg_periapsis: f64,
    pub long_asc_node: Option<f64>,
    pub body_type: Option<String>,
}

impl SolBodies {
    pub async fn get() -> Result<Self, Error> {
        let fetch_client = Client::new();
        let get_response = fetch_client.get(SOL_BODIES_URL).send().await.unwrap();
        let bodies = get_response.json::<Self>().await?;

        Ok(bodies)
    }
}

#[derive(Deserialize)]
pub struct SolBodyMoon {
    pub moon: String,
    pub rel: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolBodyMass {
    pub mass_value: f64,
    pub mass_exponent: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolBodyVol {
    pub vol_value: f64,
    pub vol_exponent: u8,
}

//The Planet which an object from the Sol Solar System (say a satelite[moon]) is orbiting
#[derive(Deserialize)]
pub struct SolBodyAroundPlanet {
    pub planet: String,
    pub rel: String,
}
