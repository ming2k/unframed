use std::fs::{self, File};
use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, RgbaImage};
use unframed::app::App;
use unframed::config::{BackgroundMode, Config};
use unframed::image_loader::load_image_from_bytes;
use unframed::navigator::Navigator;
use unframed::ppm::parse_pnm;
use unframed::svg::render_svg;
use unframed::viewport::{ScalingMode, UpscalingMethod};

#[test]
fn test_ppm_all_variants() {
    // 1. P1: ASCII PBM
    let p1 = b"P1\n# PBM ASCII\n3 2\n1 0 1\n0 1 0";
    let img1 = parse_pnm(p1).expect("P1");
    assert_eq!(img1.width, 3);
    assert_eq!(img1.height, 2);
    assert_eq!(&img1.rgba[0..4], &[0, 0, 0, 255]); // 1 = black
    assert_eq!(&img1.rgba[4..8], &[255, 255, 255, 255]); // 0 = white

    // 2. P2: ASCII PGM
    let p2 = b"P2\n2 1\n100\n50 100";
    let img2 = parse_pnm(p2).expect("P2");
    assert_eq!(img2.width, 2);
    assert_eq!(img2.height, 1);
    assert_eq!(&img2.rgba[0..4], &[127, 127, 127, 255]);
    assert_eq!(&img2.rgba[4..8], &[255, 255, 255, 255]);

    // 3. P3: ASCII PPM
    let p3 = b"P3\n1 2\n255\n255 0 0\n0 255 0";
    let img3 = parse_pnm(p3).expect("P3");
    assert_eq!(img3.width, 1);
    assert_eq!(img3.height, 2);
    assert_eq!(&img3.rgba[0..4], &[255, 0, 0, 255]);
    assert_eq!(&img3.rgba[4..8], &[0, 255, 0, 255]);

    // 4. P4: Binary PBM
    let p4 = b"P4\n4 1\n\xF0"; // first 4 bits are 1111 -> all black
    let img4 = parse_pnm(p4).expect("P4");
    assert_eq!(img4.width, 4);
    assert_eq!(img4.height, 1);
    for x in 0..4 {
        assert_eq!(&img4.rgba[x * 4..(x + 1) * 4], &[0, 0, 0, 255]);
    }

    // 5. P5: Binary PGM
    let p5 = b"P5\n2 1\n255\n\x80\xFF";
    let img5 = parse_pnm(p5).expect("P5");
    assert_eq!(img5.width, 2);
    assert_eq!(img5.height, 1);
    assert_eq!(&img5.rgba[0..4], &[128, 128, 128, 255]);
    assert_eq!(&img5.rgba[4..8], &[255, 255, 255, 255]);

    // 6. P6: Binary PPM
    let p6 = b"P6\n2 1\n255\n\xFF\x00\x00\x00\x00\xFF";
    let img6 = parse_pnm(p6).expect("P6");
    assert_eq!(img6.width, 2);
    assert_eq!(img6.height, 1);
    assert_eq!(&img6.rgba[0..4], &[255, 0, 0, 255]);
    assert_eq!(&img6.rgba[4..8], &[0, 0, 255, 255]);

    // 7. P7: Binary PAM
    let p7 = b"P7\nWIDTH 2\nHEIGHT 1\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n\xFF\x00\x00\x80\x00\xFF\x00\xFF";
    let img7 = parse_pnm(p7).expect("P7 PAM");
    assert_eq!(img7.width, 2);
    assert_eq!(img7.height, 1);
    assert_eq!(&img7.rgba[0..4], &[255, 0, 0, 128]);
    assert_eq!(&img7.rgba[4..8], &[0, 255, 0, 255]);
}

#[test]
fn test_svg_rendering_and_dimensions() {
    let svg = br##"
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="200" height="200">
        <rect width="100" height="100" fill="#0000ff"/>
    </svg>
    "##;

    let img = render_svg(svg, None).expect("SVG render");
    assert_eq!(img.width, 200);
    assert_eq!(img.height, 200);
    assert_eq!(&img.rgba[0..4], &[0, 0, 255, 255]);

    let scaled = render_svg(svg, Some((400, 400))).expect("SVG custom scaled");
    assert_eq!(scaled.width, 400);
    assert_eq!(scaled.height, 400);
}

#[test]
fn test_gif_animation_loading_and_tick() {
    // Generate valid 2-frame animated GIF
    let mut gif_bytes = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut gif_bytes);
        let frame1_buf = RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 255]));
        let frame2_buf = RgbaImage::from_pixel(2, 2, image::Rgba([0, 255, 0, 255]));
        let f1 = Frame::from_parts(frame1_buf, 0, 0, Delay::from_numer_denom_ms(50, 1));
        let f2 = Frame::from_parts(frame2_buf, 0, 0, Delay::from_numer_denom_ms(50, 1));
        encoder.encode_frame(f1).expect("encode frame 1");
        encoder.encode_frame(f2).expect("encode frame 2");
    }

    let mut img = load_image_from_bytes(&gif_bytes, Some("anim.gif")).expect("decode gif");
    assert!(img.is_animated());
    assert_eq!(img.frames.len(), 2);
    assert_eq!(img.current_frame, 0);

    // Frame delay is 50ms
    // Ticking 30ms: frame does not advance yet
    let changed = img.tick_animation(30.0);
    assert!(!changed);
    assert_eq!(img.current_frame, 0);

    // Ticking another 30ms: frame advances to 1
    let changed = img.tick_animation(30.0);
    assert!(changed);
    assert_eq!(img.current_frame, 1);

    // Manual next frame
    img.next_frame();
    assert_eq!(img.current_frame, 0);
}

#[test]
fn test_navigator_recursive_and_image_detection() {
    let temp_dir = std::env::temp_dir().join("unframed_test_nav");
    let sub_dir = temp_dir.join("subdir");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&sub_dir).expect("create temp dirs");

    File::create(temp_dir.join("img1.png")).unwrap();
    File::create(temp_dir.join("not_an_img.txt")).unwrap();
    File::create(sub_dir.join("img2.svg")).unwrap();
    File::create(sub_dir.join("img3.ppm")).unwrap();

    let mut nav = Navigator::new();
    nav.add(temp_dir.to_str().unwrap(), true);

    assert_eq!(nav.len(), 3);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_app_full_command_suite() {
    let temp_dir = std::env::temp_dir().join("unframed_test_cmd_suite");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let f1 = temp_dir.join("test1.ppm");
    let f2 = temp_dir.join("test2.ppm");
    fs::write(&f1, b"P6\n1 1\n255\n\xFF\x00\x00").unwrap();
    fs::write(&f2, b"P6\n1 1\n255\n\x00\xFF\x00").unwrap();

    let config = Config::default();
    let mut app = App::new(config, vec![], None, vec![], vec![]);

    app.execute_command(&format!("open {} {}", f1.display(), f2.display()));
    assert_eq!(app.navigator.len(), 2);
    assert!(app.current_image.is_some());

    app.execute_command("goto 2");
    assert_eq!(app.navigator.index(), 1);

    app.execute_command("prev");
    assert_eq!(app.navigator.index(), 0);

    app.execute_command("next");
    assert_eq!(app.navigator.index(), 1);

    app.execute_command("zoom actual");
    assert_eq!(app.viewport.scale, 1.0);

    app.execute_command("zoom 2");
    assert!(app.viewport.scale > 1.0);

    let old_x = app.viewport.x;
    let old_y = app.viewport.y;
    app.execute_command("pan 100 -50");
    assert!((app.viewport.x - (old_x + 100.0)).abs() < 1e-4);
    assert!((app.viewport.y - (old_y - 50.0)).abs() < 1e-4);

    app.execute_command("center");
    app.execute_command("reset");
    assert_eq!(app.viewport.rotation, 0.0);
    assert!(!app.viewport.mirrored);

    app.execute_command("rotate to 180");
    assert_eq!(app.viewport.rotation, 180.0);

    app.execute_command("flip vertical");
    assert!(app.viewport.mirrored);

    app.execute_command("scaling shrink");
    assert_eq!(app.viewport.scaling_mode, ScalingMode::Shrink);

    app.execute_command("upscaling linear");
    assert_eq!(app.viewport.upscaling_method, UpscalingMethod::Linear);

    app.execute_command("slideshow 10");
    assert_eq!(app.config.slideshow_duration, 10.0);

    app.execute_command("background 123456");
    assert!(matches!(app.config.background, BackgroundMode::Solid(_)));

    app.execute_command("background checks");
    assert_eq!(app.config.background, BackgroundMode::Checks);

    app.execute_command("alias mynext next");
    app.execute_command("mynext");
    assert_eq!(app.navigator.index(), 0);

    app.execute_command("close");
    assert_eq!(app.navigator.len(), 1);

    app.execute_command("close all");
    assert_eq!(app.navigator.len(), 0);

    let _ = fs::remove_dir_all(&temp_dir);
}
