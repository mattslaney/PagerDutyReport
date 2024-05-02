{
    description = "Dev Flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self , nixpkgs ,... }: let
        system = "x86_64-linux";
    in {
        devShells."${system}".default = let
            pkgs = import nixpkgs {
                inherit system;
            };
        in pkgs.mkShell {
            packages = with pkgs; [
                nodejs_20
                typescript
            ];

            shellHook = ''
                export PATH="$HOME/.cargo/bin:$PATH"
                echo "Dev Flake"
                echo "node `${pkgs.nodejs}/bin/node --version`"
                '';
        };
    };
}
