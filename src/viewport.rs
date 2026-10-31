//! Viewport transforms, scaling, rotation, zoom, and pan calculations.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScalingMode {
    None,
    Shrink,
    #[default]
    Full,
    Crop,
}

impl ScalingMode {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "none" | "actual" => Some(ScalingMode::None),
            "shrink" | "down" => Some(ScalingMode::Shrink),
            "full" | "fit" => Some(ScalingMode::Full),
            "crop" | "fill" => Some(ScalingMode::Crop),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ScalingMode::None => "none",
            ScalingMode::Shrink => "shrink",
            ScalingMode::Full => "full",
            ScalingMode::Crop => "crop",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ScalingMode::None => ScalingMode::Shrink,
            ScalingMode::Shrink => ScalingMode::Full,
            ScalingMode::Full => ScalingMode::Crop,
            ScalingMode::Crop => ScalingMode::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpscalingMethod {
    #[default]
    Linear,
    NearestNeighbour,
}

impl UpscalingMethod {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "linear" => Some(UpscalingMethod::Linear),
            "nearest_neighbour" | "nearest" => Some(UpscalingMethod::NearestNeighbour),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UpscalingMethod::Linear => "linear",
            UpscalingMethod::NearestNeighbour => "nearest_neighbour",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            UpscalingMethod::Linear => UpscalingMethod::NearestNeighbour,
            UpscalingMethod::NearestNeighbour => UpscalingMethod::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomSource {
    Mouse,
    Keyboard,
}

pub struct Viewport {
    pub scale: f64,
    pub rotation: f64,
    pub mirrored: bool,
    pub x: f64,
    pub y: f64,
    pub buffer_width: f64,
    pub buffer_height: f64,
    pub pan_factor_x: f64,
    pub pan_factor_y: f64,
    pub scaling_mode: ScalingMode,
    pub upscaling_method: UpscalingMethod,
    pub locked: bool,
    pub playing: bool,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            scale: 1.0,
            rotation: 0.0,
            mirrored: false,
            x: 0.0,
            y: 0.0,
            buffer_width: width.max(1.0),
            buffer_height: height.max(1.0),
            pan_factor_x: 0.5,
            pan_factor_y: 0.5,
            scaling_mode: ScalingMode::Full,
            upscaling_method: UpscalingMethod::Linear,
            locked: false,
            playing: true,
        }
    }

    pub fn set_window_size(&mut self, width: f64, height: f64, img_w: u32, img_h: u32) {
        self.buffer_width = width.max(1.0);
        self.buffer_height = height.max(1.0);
        if !self.locked && img_w > 0 && img_h > 0 {
            self.rescale(img_w, img_h);
        }
    }

    pub fn rescale(&mut self, img_w: u32, img_h: u32) {
        if img_w == 0 || img_h == 0 {
            return;
        }

        match self.scaling_mode {
            ScalingMode::None => {
                self.scale_to_actual(img_w, img_h);
            }
            ScalingMode::Shrink => {
                if self.buffer_width >= img_w as f64 && self.buffer_height >= img_h as f64 {
                    self.scale_to_actual(img_w, img_h);
                } else {
                    self.scale_to_window(img_w, img_h);
                }
            }
            ScalingMode::Full => {
                self.scale_to_window(img_w, img_h);
            }
            ScalingMode::Crop => {
                self.crop_to_window(img_w, img_h);
            }
        }
    }

    pub fn scale_to_actual(&mut self, img_w: u32, img_h: u32) {
        self.scale = 1.0;
        self.center(img_w, img_h);
        self.locked = true;
    }

    pub fn scale_to_window(&mut self, img_w: u32, img_h: u32) {
        if img_w == 0 || img_h == 0 {
            return;
        }

        let win_aspect = self.buffer_width / self.buffer_height;
        let img_aspect = img_w as f64 / img_h as f64;

        if win_aspect > img_aspect {
            self.scale = self.buffer_height / img_h as f64;
        } else {
            self.scale = self.buffer_width / img_w as f64;
        }

        self.center(img_w, img_h);
        self.locked = false;
    }

    pub fn crop_to_window(&mut self, img_w: u32, img_h: u32) {
        if img_w == 0 || img_h == 0 {
            return;
        }

        let win_aspect = self.buffer_width / self.buffer_height;
        let img_aspect = img_w as f64 / img_h as f64;

        if win_aspect > img_aspect {
            self.scale = self.buffer_width / img_w as f64;
        } else {
            self.scale = self.buffer_height / img_h as f64;
        }

        self.center(img_w, img_h);
        self.locked = false;
    }

    pub fn center(&mut self, img_w: u32, img_h: u32) {
        let scaled_w = img_w as f64 * self.scale;
        let scaled_h = img_h as f64 * self.scale;

        let rem_x = self.buffer_width - scaled_w;
        let rem_y = self.buffer_height - scaled_h;

        if rem_x > 0.0 {
            self.x = rem_x * 0.5;
        } else {
            self.x = rem_x * self.pan_factor_x;
        }

        if rem_y > 0.0 {
            self.y = rem_y * 0.5;
        } else {
            self.y = rem_y * self.pan_factor_y;
        }
    }

    pub fn move_by(&mut self, dx: f64, dy: f64, img_w: u32, img_h: u32) {
        self.x += dx;
        self.y += dy;
        self.locked = true;

        let scaled_w = img_w as f64 * self.scale;
        let scaled_h = img_h as f64 * self.scale;

        self.x = self.x.clamp(-scaled_w, self.buffer_width);
        self.y = self.y.clamp(-scaled_h, self.buffer_height);
    }

    pub fn scale_by(
        &mut self,
        factor: f64,
        src: ZoomSource,
        mouse_x: f64,
        mouse_y: f64,
        img_w: u32,
        img_h: u32,
    ) {
        if img_w == 0 || img_h == 0 {
            return;
        }

        let prev_scale = self.scale;
        let (focus_x, focus_y) = match src {
            ZoomSource::Mouse => (mouse_x - self.x, mouse_y - self.y),
            ZoomSource::Keyboard => (
                self.scale * img_w as f64 * 0.5,
                self.scale * img_h as f64 * 0.5,
            ),
        };

        self.scale = (self.scale * factor).clamp(0.01, 100.0);
        let ratio = self.scale / prev_scale;

        self.x += focus_x - (focus_x * ratio);
        self.y += focus_y - (focus_y * ratio);

        self.locked = true;
    }

    pub fn zoom(
        &mut self,
        amount: f64,
        src: ZoomSource,
        mouse_x: f64,
        mouse_y: f64,
        img_w: u32,
        img_h: u32,
    ) {
        let factor = 1.04_f64.powf(amount);
        self.scale_by(factor, src, mouse_x, mouse_y, img_w, img_h);
    }

    pub fn rotate_by(&mut self, degrees: f64) {
        self.rotation = (self.rotation + degrees) % 360.0;
    }

    pub fn rotate_to(&mut self, degrees: f64) {
        self.rotation = degrees % 360.0;
    }

    pub fn flip_h(&mut self) {
        self.mirrored = !self.mirrored;
    }

    pub fn flip_v(&mut self) {
        self.mirrored = !self.mirrored;
        self.rotation = (self.rotation - 180.0) % 360.0;
    }

    pub fn reset_transform(&mut self) {
        self.rotation = 0.0;
        self.mirrored = false;
    }

    pub fn reset(&mut self, img_w: u32, img_h: u32) {
        self.reset_transform();
        self.locked = false;
        self.rescale(img_w, img_h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_scaling_modes() {
        let mut vp = Viewport::new(800.0, 600.0);
        // Image is 400x300 (aspect 4:3, window aspect 4:3)
        vp.scaling_mode = ScalingMode::Full;
        vp.rescale(400, 300);
        assert!((vp.scale - 2.0).abs() < 1e-4);
        assert!((vp.x - 0.0).abs() < 1e-4);
        assert!((vp.y - 0.0).abs() < 1e-4);

        vp.scaling_mode = ScalingMode::None;
        vp.rescale(400, 300);
        assert_eq!(vp.scale, 1.0);
        assert_eq!(vp.x, 200.0); // (800 - 400)/2
        assert_eq!(vp.y, 150.0); // (600 - 300)/2
    }

    #[test]
    fn test_viewport_zoom() {
        let mut vp = Viewport::new(800.0, 600.0);
        vp.scale_to_actual(200, 200);
        let s0 = vp.scale;
        vp.zoom(5.0, ZoomSource::Keyboard, 400.0, 300.0, 200, 200);
        assert!(vp.scale > s0);
    }

    #[test]
    fn test_viewport_rotation_flip() {
        let mut vp = Viewport::new(800.0, 600.0);
        vp.rotate_by(90.0);
        assert_eq!(vp.rotation, 90.0);
        vp.flip_h();
        assert!(vp.mirrored);
        vp.reset_transform();
        assert_eq!(vp.rotation, 0.0);
        assert!(!vp.mirrored);
    }
}
