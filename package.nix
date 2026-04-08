{
  pkgs,
  lib,
  craneLib,
}:

let
  pname = "spacenav-cockpit";

  src = lib.cleanSourceWith {
    src = ./.;
    name = "source";
    filter =
      let
        filterCSources = path: _type: builtins.match ".*(c|h)$" path != null;
        filterAssets = path: _type: builtins.match ".*(webp|png)$" path != null;

        allFilters = [
          craneLib.filterCargoSources
          filterCSources
          filterAssets
        ];
      in
      path: type: builtins.any (filterFunc: filterFunc path type) allFilters;
  };

  commonArgs = {
    inherit src pname;
    strictDeps = true;
  };

  # all dependencies, without our code -> make caching effective
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # https://github.com/iced-rs/iced/blob/master/DEPENDENCIES.md
  dlopenLibraries = with pkgs; [
    libxkbcommon

    # GPU backend
    vulkan-loader
    # libGL

    # Window system
    wayland
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
  ];
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    doCheck = false; # the test 'toml::tests::test_read_and_write_toml' is currently broken

    env = {
      LIBCLANG_PATH = "${pkgs.lib.getLib pkgs.libclang}/lib";
    };

    nativeBuildInputs = with pkgs; [
      makeWrapper
    ];

    postInstall = ''
      # Install app icon file
      install -Dm444 spacenav-cockpit/assets/app-icon_256x256.png \
        $out/share/icons/hicolor/256x256/apps/spacenav-cockpit.png
      # Install the desktop file
      mkdir -p $out/share/applications
      cat > $out/share/applications/${pname}.desktop << EOF
      [Desktop Entry]
      Type=Application
      Name=SpaceNav Cockpit
      Comment=Application to comfortably change spacenav daemon's settings.
      Exec=$out/bin/${pname}
      Icon=${pname}
      Categories=Utility;
      Terminal=false
      EOF
    '';

    postFixup = ''
      wrapProgram $out/bin/${pname} \
        --suffix LD_LIBRARY_PATH : ${lib.makeLibraryPath dlopenLibraries}
    '';
  }
)
