{ pkgs }: {
	deps = [
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
    pkgs.pkg-config
    pkgs.openssl
	];
}
