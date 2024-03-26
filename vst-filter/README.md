# VST Distortion Filter
  This is a little example of how to produce a VST plugin that receives as input a tune or a sound
  and then distortionates the sound altering the gain of the samples. It is recommended to run it with low    
  volume or otherwise it can produce annoyance in the ears.    

## Table of contents
- [Development](#development)
- [Dependencies](#dependencies)
- [Usage](#usage)

## Development    
  The plugin was developed with Rust using the [Nih-Plug](https://github.com/robbert-vdh/nih-plug) framework which provides an easy way to produce 
  plugins in VST and CLAP formats, the frameworks uses a crate included in its workspace called `xtask` which is in charge of producing the bundles at compilation time.    
  The Reaper and Ardour DAWs were used to test the plugin in Windows and linux respectively, but it should work in any other as it is DAW-agnostic.
  
## Dependencies 

  - Git
  - Rust
  
## Usage 
  
  Inside the root directory run the command: 
  ```
  cargo xtask bundle vst-filter --release
  ```
  This will create the file `vst-filter.vst3` inside the `target\bundled\` folder created by the command.
  It is recommended to have a folder in the system where all the VST plugins are stored and then copy paste the produced file inside of it.
  Another option is to set the path in the DAW where the plugin is generated but is not optimal. 
 
