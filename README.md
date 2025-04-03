Funpar: Parallelized Image Processing in Rust

This project will be applying a Rust-based image processing tool that applies filters like blur, sharpen, and brightness adjustments (lightening or darkening) while also using concurrent programming to run way faster than traditional single-threaded programs.

How to Use It

Find the main file: Go to /project/src/main.rs. This is where the magic happens.

Set your image path: Assign your image's file path to the variable s so the program knows what to process.

Run the program: Either press the run button in your editor or compile and execute it from the terminal.

Wait for processing:

The program will apply all filters in order.

The processed images will be saved outside the src folder.

Tweak the filters:

Adjust blur_radius to control the blur strength.

Adjust brightening_factor to lighten the image.

Adjust darkening_factor to darken it.

Filters & Performance Breakdown

Blur Filter

What it does: Makes the image smoother by averaging nearby colors.

How it works:

Loops through each pixel and blends it with surrounding ones based on blur_radius.

More blur = more calculations.

Performance:

Loops over all pixels: O(w × h)

Blending neighbors: O((2 × blur_radius + 1)²)

Total Complexity: O(w × h × (2 × blur_radius + 1)²)

Parallelization: Uses Rayon to distribute computations across multiple threads.

Sharpen Filter

What it does: Enhances edges and details in the image.

How it works:

Uses a 3x3 kernel to tweak each pixel.

Performance:

Loops through all pixels: O(w × h)

Applies kernel: O(1)

Total Complexity: O(w × h)

Parallelization: Processed using Rayon for improved efficiency.

Darken Filter

What it does: Makes the image darker.

How it works:

Multiplies pixel RGB values by a darkening factor.

Performance:

Loops through all pixels: O(w × h)

Adjusts RGB values: O(1)

Total Complexity: O(w × h)

Parallelization: Runs in parallel to accelerate execution.

Brighten Filter

What it does: Makes the image brighter.

How it works:

Multiplies pixel RGB values by a brightening factor.

Performance:

Loops through all pixels: O(w × h)

Adjusts RGB values: O(1)

Total Complexity: O(w × h)

Parallelization: Uses multi-threading for faster processing.

Wrapping Up:

Parallel makes image processing fast by leveraging Rust’s Rayon library. Whether you’re blurring, sharpening, or adjusting brightness, it processes images way quicker than traditional methods. Plus, you can fine-tune the filters easily!
