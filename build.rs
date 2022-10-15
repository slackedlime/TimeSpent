fn main() {
	if cfg!(target_os = "windows") && !cfg!(debug_assertions) {
		let mut res = winres::WindowsResource::new();
		res.set_icon("imgs/hummingbird_new.ico");

		if let Err(e) = res.compile() {
			eprintln!("Could not attach Icon to executable");
			eprintln!("Error: {}\n", e);
		}
	}
}