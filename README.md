# Simple API written in Rust
It calculates the average gas usage based on a few factors or returns a random probability of a unit injector failure.

## How to run
Easiest is to just clone the repository locally or download it. Then run it either via `cargo run` or the `rust-analyzer` VS Code extension.

## Endpoints

##### /calculateDisselUsageForDistance
Query parameters (all are u32 and required):
- distance
- yearOfProduction
- fuelUsagePer100KM

##### /probabilityOfUnitInjectorFail
Required query parameters:
- VIN

Important note: this needs to be a valid VIN number. Otherwise, you will get a 400 Bad Request error. You can use this test VIN number for testing: `1M8GDM9AXKP042788`.

## Possible responses
- /calculateDisselUsageForDistance
	- 200 returns a JSON with `fuelUsage` as a number.
- /probabilityOfUnitInjectorFail
	- 200 returns a JSON with `failProbability` as a number in a range of 0.00 - 1.00.
- Errors
	- Any invalid request will return a 400 Bad Request with a clear error message explaining what went wrong.
