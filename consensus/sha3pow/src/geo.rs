use sp_core::{ H256 };
use noise::{ OpenSimplex, Seedable, NoiseFn };
use rand::{ Rng, SeedableRng };

pub fn gradient_noise(hash: &H256) -> OpenSimplex {
	let mut seed = 0;
	for c in hash.to_string().chars() {
		seed += c as u32;
	}
	let generator = OpenSimplex::new();
	generator.set_seed(seed);
	return generator;
}

pub fn minutes_elapsed_since(timestamp: u64) -> u64 {
	let now = std::time::SystemTime::now();
	let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
	let in_seconds = since_the_epoch.as_secs();
	return (in_seconds - timestamp) / 60;
}

// TODO: generate simulated lon and lat
pub fn get_geolocation_by_ip(ip: &str) -> (f64, f64) {
	let mut seed: u64 = 0;
	for c in ip.chars() {
		seed += c as u64;
	}
	let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
	let lat = rng.gen_range(-90.0..90.0);
	let lon = rng.gen_range(-180.0..180.0);
	return (lat, lon);
}

pub fn node_is_on_mining_zone(hash: &H256, timestamp: u64, ip: &str) -> bool {
	let generator = gradient_noise(hash);
	let access = 1.0 - ((minutes_elapsed_since(timestamp) as f64) + 1.0) / 10.0;
	let mut canvas = [[false; 180]; 360];
	let mut xoff: f64 = 0.0;
	let noise_scale = 0.01;

	for x in 0..360 {
		let mut yoff: f64 = 0.0;
		for y in 0..180 {
			let n = generator.get([xoff, yoff]);
			canvas[x][y] = n > access;
			yoff += noise_scale;
		}
		xoff += noise_scale;
	}
	let (lon, lat) = get_geolocation_by_ip(ip);
	let is_valid_zone = canvas[lon as usize][lat as usize];

	return is_valid_zone;
}