$uhd_root=$env:UHD_ROOT;
$uhd_include = Join-Path -Path $uhd_root -ChildPath "include";
$uhd_h = Join-Path -Path $uhd_include -ChildPath "uhd.h";
$dst = Join-Path -Path $PSScriptRoot -ChildPath "src" "bindings.rs";
$cmd = "bindgen ""$uhd_h"" -o $dst -- -I ""$uhd_include"" --target=x86_64-pc-windows-msvc";
Invoke-Expression $cmd;