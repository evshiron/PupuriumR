@ECHO OFF

cargo build

mkdir dist

copy %~dp0\target\debug\pupurium_r.dll %~dp0\dist\com.github.res.pupurium_r.dll
copy %~dp0\com.github.res.pupurium_r.json %~dp0\dist\com.github.res.pupurium_r.json
