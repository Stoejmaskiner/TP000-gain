cargo xtask bundle gain 
Remove-Item -Recurse -Path 'C:\Program Files\Common Files\VST3\Stoej Gain.vst3'
Copy-Item -Recurse -Path '.\target\bundled\Stoej Gain.vst3' -Destination 'C:\Program Files\Common Files\VST3\'
fl64.exe