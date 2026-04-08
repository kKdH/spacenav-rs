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
        filterAssets = path: _type: builtins.match ".*webp$" path != null;

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

    postFixup = ''
      wrapProgram $out/bin/${pname} \
        --suffix LD_LIBRARY_PATH : ${lib.makeLibraryPath dlopenLibraries}
    '';
  }
)
