use *;

use csv;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Version {
	pub channel: String,
	pub version: String,
	pub platform: String,
	pub hash: String,
	#[serde(default)]
	count: u32,
}

impl Version {
	fn get_enum_name(&self) -> String {
		let mut res = String::new();
		res.push_str(&self.platform.replace(' ', "_"));
		let ver = self.version.split(' ').next().unwrap();
		for num in ver.split('.') {
			if num != "?" {
				res.push('_');
				res.push_str(&format!("{}", num));
			} else {
				res.push('_');
				res.push('X');
			}
		}
		if self.channel != "Stable" {
			res.push('_');
			res.push_str(&self.channel);
		}
		if self.count != 0 {
			res.push_str("__");
			res.push_str(&self.count.to_string());
		}
		res
	}

	fn get_sign_array(&self) -> String {
		let mut res = String::new();
		for b in ::base64::decode(&self.hash).unwrap() {
			if !res.is_empty() {
				res.push_str(", ");
			}
			res.push_str(&format!("{:#x}", b));
		}
		res
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VersionKey {
	pub channel: String,
	pub version: String,
	pub platform: String,
}

impl VersionKey {
	fn new(v: &Version) -> Self {
		Self {
			channel: v.channel.clone(),
			version: v.version.split(' ').next().unwrap().to_string(),
			platform: v.platform.clone(),
		}
	}
}

#[derive(Template)]
#[TemplatePath = "src/VersionDeclarations.tt"]
#[derive(Default, Debug)]
pub struct Versions(Vec<Version>);

impl Declaration for Versions {
	type Dep = ();

	fn get_filename() -> &'static str {
		"Versions.csv"
	}

	fn parse_from_read(read: &mut Read, (): Self::Dep) -> Self {
		let mut table = csv::Reader::from_reader(read);
		let mut vs = Versions(
			table.deserialize().collect::<Result<Vec<_>, _>>().unwrap(),
		);

		// Add count if necessary
		let mut counts: HashMap<_, u32> = HashMap::new();
		for v in &vs.0 {
			let key = VersionKey::new(v);
			*counts.entry(key).or_default() += 1;
		}
		counts.retain(|_, c| *c > 1);

		for v in vs.0.iter_mut().rev() {
			let key = VersionKey::new(v);
			if let Some(count) = counts.get_mut(&key) {
				v.count = *count;
				*count -= 1;
			}
		}

		vs
	}
}
