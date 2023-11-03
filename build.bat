
@echo off

cls

if not exist build\NUL mkdir build

pushd build
	rustc ..\rcopy.rs
popd

copy build\rcopy.exe rcopy.exe > NUL