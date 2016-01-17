GIR = gir/target/release/gir
GTK_GIR = gir-files/Gtk-3.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs

src/auto/mod.rs : Gir.toml $(GIR) $(GTK_GIR)
	$(GIR) -c Gir.toml

$(GIR) : gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
	cd gir && cargo build --release

gir/Cargo.toml $(GTK_GIR) :
	git submodule update --init
