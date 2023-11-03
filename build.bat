
@echo off

cls

pushd build
	rustc ..\rcopy.rs
popd

copy build\rcopy.exe rcopy.exe > NUL