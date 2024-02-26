with import <nixpkgs> {};
mkShell rec {
	nativeBuildInputs = with buildPackages; [
		rustup
		alsa-lib
		pkg-config
		systemd
		wayland
		libGL
		libxkbcommon 
	];
	LD_LIBRARY_PATH = "${lib.makeLibraryPath nativeBuildInputs}";
}
