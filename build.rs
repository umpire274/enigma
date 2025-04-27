#[cfg(target_os = "windows")]
fn main() {
	embed_resource::compile("packaging/windows/icon.rc");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
