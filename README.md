# Dimension Swapped
I wrote this program with inspiration from [Mike Germain's "Steamed Hams but the Spatial x Dimension is Swapped with the Temporal"](https://www.youtube.com/watch?v=SETypddWfsM). This will let you swap the X and Time dimension of any video you can download, with some steps.

**Note: I will not be teaching you how to install ffmpeg, the Rust toolchain, open a terminal in a certain folder. or get video stats! Please figure that out for yourself!**

## Instructions
1. Copy the video you want to mangle into the project directory and record it's framerate
2. Convert the video into an image sequence in the `Source` folder by running `ffmpeg -i [Video file] Source/%06d.png` and wait for it to finish.
3. Run `cargo run --release` and wait for the program to build and run.
4. Convert the image sequence in the `Destination` folder by running `ffmpeg -r [framerate] -f image2 -i Destination/%06d.png -vcodec libx264 -pix_fmt yuv420p Processed.mp4`, where `[framerate]` is the framerate of the original video
5. Finish editing with your favorite video editor