use image::{DynamicImage, ImageBuffer, ImageReader, GenericImageView};
use std::{env, str::FromStr};
use std::fs::File;
use std::io::{Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    println!("{}", argc);
    if argc < 3 {
        println!("No input image\nUsage: ./gen <image file> <output swift>");
        return Ok(());
    }
        let mut output_s = args[2].clone();
        if !output_s.ends_with(".swift") {
            output_s.push_str(".swift");
        }
        let mut output_f = File::create(&output_s)
            .map_err(|e| format!("Failed to create output file '{}': {}", output_s, e))?;


        match std::fs::metadata(&args[1]) {
            Ok(meta) => println!("Input file '{}' size: {} bytes", &args[1], meta.len()),
            Err(e) => println!("Could not read metadata for '{}': {}", &args[1], e),
        }

        let img: DynamicImage = ImageReader::open(&args[1])
            .map_err(|e| format!("Failed to open image '{}': {}", args[1], e))?
            .decode()
            .map_err(|e| format!("Failed to decode image '{}': {}", args[1], e))?;

    let (width, height) = img.dimensions();
    println!("圖片大小: {}x{}", width, height);


    let mut img_swift:String = String::from_str(r#"import SwiftUI
import SwiftData
struct img_view: View {
    var body: some View {

        VStack {
        "#).unwrap();

    let end_swift: String = String::from_str(r#" 
        }
        
    }
}

@main
struct app: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(sharedModelContainer)
    }
}"#).unwrap();
    // 逐 pixel 拿顏色
    for y in 0..height {
        img_swift.push_str("\tHStack{\n");
        for x in 0..width {
            let pixel = img.get_pixel(x, y); // Rgba<u8>
            let [r, g, b, a] = pixel.0;
            // Normalize to 0.0 - 1.0 for Swift Color
            let rf = (r as f32) / 255.0;
            let gf = (g as f32) / 255.0;
            let bf = (b as f32) / 255.0;
            let af: f32 = (a as f32) / 255.0;

            img_swift.push_str(&format!(
                "\t\t\t\tRect()\n\t\t\t\t\t.fill(Color(red: {:.3}, green: {:.3}, blue: {:.3}, opacity: {:.3}))\n\t\t\t\t\t.frame(width: 1, height: 1);\n",
                rf, gf, bf, af
            ));
        }
        img_swift.push_str("\t\t\t\t}\n")
    }
    img_swift.push_str(&end_swift);

    output_f.write_all(img_swift.as_bytes()).unwrap();    


    Ok(())
}
