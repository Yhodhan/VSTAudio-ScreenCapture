# Windows Screenshot

The general idea of the crate is to directly use the windows api to get a bitmap of the screen or screens
and then produce a png o bmp format screenshot, jpg format is not supported in this implementation mainly because it requires more data to be generated.

## Usage 
  Clone the repository
  ```
  git clone https://github.com/hixion/VSTAudio-ScreenCapture/tree/main/screenshot`
  ```
  
  then run 

  ```
  cargo run
 
  ```
 The program will generate the image with the specified format inside the root directory
