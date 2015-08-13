rustlib = target/release/libpoints.dylib

$(rustlib): points.rs
	cargo build --release

test: test.rb points.rb $(rustlib)
	ruby $<
