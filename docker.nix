{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  hello = import ./hello.nix { inherit sources pkgs; };

  name = "anderscs/hello";
  tag = "latest";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ hello ];

  config = {
    Cmd = [ "/bin/backend" ];
    Env = [ "ROCKET_PORT=5000" ];
    WorkingDir = "/";
  };
}
