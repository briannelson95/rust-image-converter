# ConvertIt

Desktop image converter built in Rust. 

### Table of Contents
- [Installation](#installation)
- [To Do](#to-do)

## Installation
This is a WIP and has not been fully released. To run this program right now, you will need to have rust installed on your machine; [learn how to install rust](https://www.rust-lang.org/tools/install). If you would like to use this program as-is, follow these instructions:

1. Clone this repo `git clone https://github.com/briannelson95/rust-image-converter` or click on "Code" and choose your preferred cloning method.
2. Open Terminal or Command Line
3. Navigate to the directory `cd rust-image-converter`
4. Run `cargo build` and then `cargo run`

## To Do
### Single Conversion
- [x] Get application running
- [x] Basic conversion, convert jpg to webp
- [x] Allow users to browse for image file and output folder
- [x] Allow users to change name of converted file
- [x] Allow users to choose the type of conversion from dropdown
- [x] Add checkbox that will open folder of converted file when conversion is complete
- [x] Add all image conversion types
    - [ ] Convert image from HEIC to any other format
- [ ] Add image preview for single image conversion
- [ ] Add pdf to image types conversion

### Bulk Conversion
- [ ] Add tab navigation to go from single to bulk
- [ ] Put converted files into a newly created dir `date/time-conversion`
- [ ] Checkbox to open the newly created dir
- [ ] Replace image preview with list of file paths 
    - [ ] Allow user to click to select and press backspace/delete
    - [ ] Allow users to right click and choose remove image from dropdown
- [ ] Loop through all files and convert to the selected file type regardless of initial type

### Package and Distribute
- [ ] Add app icon/logo
- [ ] Package for both macOS and Windows
- [ ] Add to Releases on GitHub
- [ ] Create webpage for easy download