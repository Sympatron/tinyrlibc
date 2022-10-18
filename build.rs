use cc;

fn main() {
	// Build our snprintf substitute (which has to be C as Rust doesn't do varargs)
	cc::Build::new()
		.warnings(true)
		.extra_warnings(true)
		.flag("-std=c99")
		.file("./src/snprintf.c")
		.compile("clocal");
}
