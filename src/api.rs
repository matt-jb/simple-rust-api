#![allow(non_snake_case)]
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use chrono::Datelike;
use vin;
use rand::Rng;

#[derive(Deserialize)]
pub struct FuelUsageQuery {
    distance: u32,
    yearOfProduction: u32,
    fuelUsagePer100KM: u32,
}

#[derive(Serialize)]
pub struct FuelUsageResponse {
    fuelUsage: f32,
}

#[derive(Deserialize)]
pub struct FailQuery {
    VIN: String,
}

#[derive(Serialize)]
pub struct FailResponse {
    failProbability: f32,
}

fn throw_error(msg: &str) -> HttpResponse {
    HttpResponse::BadRequest().body(format!("{}", &msg))
}

#[get("/calculateDieselUsageForDistance")]
pub async fn get_fuel_usage(params: web::Query<FuelUsageQuery>) -> HttpResponse {
    let current_year = chrono::Utc::now().date().year() as u32;
    if params.yearOfProduction > current_year {
        throw_error("Your PeopleCar PasWagon C6 was produced in the future. Please check with your time-travel agent.")
    } else if params.fuelUsagePer100KM == 0 {
        throw_error("We're kind of behind on electrifying our vehicles. PeopleCar PasWagon C6 does, in fact, run on gas.")
    } else if params.distance == 0 {
        throw_error("Welp, you won't burn any diesel if you don't go anywhere.")
    } else {
        let fuel_usage_increase = (current_year - params.yearOfProduction) as f32 * 0.1; // 1 litre every 10 years
        let fuel_usage = (params.fuelUsagePer100KM as f32 + fuel_usage_increase) / 100.0 * params.distance as f32;
        let response = FuelUsageResponse {
            fuelUsage: fuel_usage,
        };
        HttpResponse::Ok().json(response)
    }
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn get_fail_probability(params: web::Query<FailQuery>) -> HttpResponse {
    match vin::check_validity(&params.VIN).is_ok() {
        true => {
            let random_value = (rand::thread_rng().gen_range(0.00..1.01) * 100f32).floor() / 100.0;
            let response = FailResponse {
                failProbability: random_value,
            };
            HttpResponse::Ok().json(response)
        },
        false => throw_error("VIN number is invalid. Good luck finding THAT typo.")
    }
}