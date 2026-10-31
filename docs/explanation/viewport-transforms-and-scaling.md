# Viewport Transforms and Coordinate Spaces

This article explains the mathematical foundation of Unframed's viewport transformation engine, covering scaling modes, orientation matrices, and cursor-pinned zooming geometry.

---

## 1. Coordinate Spaces

Rendering an image requires projecting pixels from local texture memory to the Wayland window surface. Unframed defines two distinct coordinate spaces:

```text
┌─────────────────────────┐          ┌───────────────────────────────────┐
│       Image Space       │          │           Window Space            │
│  Natural dimensions:    │  ──────► │  Compositor surface:              │
│  (0, 0) to (W_img, H_img)│          │  (0, 0) to (W_win, H_win)         │
└─────────────────────────┘          └───────────────────────────────────┘
```

1. **Image Space**: The unscaled, intrinsic pixel grid of the decoded bitmap or vector viewport: `(x_img, y_img) ∈ [0, W_img] × [0, H_img]`.
2. **Window Space**: The active surface coordinates allocated by the Wayland compositor via `xdg_toplevel`: `(x_win, y_win) ∈ [0, W_win] × [0, H_win]`.

---

## 2. Scaling Modes

When an image is loaded or the window is resized, the base scale factor $S$ is computed according to the active `ScalingMode`:

### 2.1 `None` (Actual Size)
Preserves 1:1 pixel mapping:
$$S = 1.0$$

### 2.2 `Shrink` (Scale Down Only)
Downscales images that exceed the window boundary, but never enlarges images that are smaller than the window:
$$S = \min\left(1.0, \frac{W_{\text{win}}}{W_{\text{img}}}, \frac{H_{\text{win}}}{H_{\text{img}}}\right)$$

### 2.3 `Full` (Fit Window)
Scales the image up or down such that the entire image is visible inside the window bounds while preserving aspect ratio:
$$S = \min\left(\frac{W_{\text{win}}}{W_{\text{img}}}, \frac{H_{\text{win}}}{H_{\text{img}}}\right)$$

### 2.4 `Crop` (Fill Window)
Scales the image such that the entire window surface is covered, cropping any overflowing edges while preserving aspect ratio:
$$S = \max\left(\frac{W_{\text{win}}}{W_{\text{img}}}, \frac{H_{\text{win}}}{H_{\text{img}}}\right)$$

---

## 3. Rotation and Aspect Inversion

Unframed supports orthogonal clockwise rotation in increments of 90 degrees ($\theta \in \{0^\circ, 90^\circ, 180^\circ, 270^\circ\}$) and horizontal/vertical flipping:

```text
           0°                   90°                  180°                 270°
     ┌───────────┐         ┌───────┐            ┌───────────┐         ┌───────┐
     │  TOP (W)  │         │       │            │  BOT (W)  │         │       │
     │           │ (H)     │   W   │ (W)        │           │ (H)     │   W   │ (W)
     │           │         │       │            │           │         │       │
     └───────────┘         └───────┘            └───────────┘         └───────┘
                              (H)                                        (H)
```

When $\theta \in \{90^\circ, 270^\circ\}$, the effective width and height of the image space are transposed:
$$W_{\text{eff}} = H_{\text{img}}, \quad H_{\text{eff}} = W_{\text{img}}$$

This ensures that fit algorithms (`Full`, `Shrink`, `Crop`) compute scale bounds against the rotated bounding box rather than the pre-rotated texture dimensions.

---

## 4. Mouse-Centered Zoom Pinning

When a user zooms in or out using the mouse scroll wheel, intuitive interaction dictates that the specific image feature directly beneath the mouse cursor must remain stationary under the cursor.

Let $(C_x, C_y)$ represent the cursor coordinates in Window Space. Let $(P_x, P_y)$ represent the current viewport pan offset, and let $S_{\text{old}}$ and $S_{\text{new}}$ represent the scale before and after zooming.

### 4.1 Derivation
1. Calculate the unscaled Image Space coordinate $(I_x, I_y)$ directly under the cursor:
   $$I_x = \frac{C_x - P_x}{S_{\text{old}}}, \quad I_y = \frac{C_y - P_y}{S_{\text{old}}}$$
2. After applying the new scale factor $S_{\text{new}}$, $(I_x, I_y)$ must project back to the same cursor coordinate $(C_x, C_y)$:
   $$C_x = P_{x,\text{new}} + I_x \cdot S_{\text{new}}$$
3. Solving for the new pan offset $P_{x,\text{new}}$:
   $$P_{x,\text{new}} = C_x - \left(\frac{C_x - P_x}{S_{\text{old}}}\right) \cdot S_{\text{new}}$$
   $$P_{y,\text{new}} = C_y - \left(\frac{C_y - P_y}{S_{\text{old}}}\right) \cdot S_{\text{new}}$$

By updating the pan offset using this exact delta, the visual anchor under the mouse cursor never jumps or drifts during continuous zooming.

---

## 5. Hardware Sampler Selection: Linear vs. Nearest

When projecting scaled pixels to the framebuffer, Vulkan utilizes texture samplers to interpolate color values:

- **`Linear` (Bilinear Filtering)**:
  Samples the four nearest texels and performs linear interpolation in both dimensions. This eliminates aliasing staircases on photographic images, high-resolution scans, and downscaled vectors.
- **`NearestNeighbour` (Point Sampling)**:
  Selects the single texel whose center is closest to the sample point. This preserves strict, hard pixel borders with zero blurring, making it the preferred sampler for pixel art, icon design, and retro game textures.

Unframed allows instantaneous runtime toggling between these samplers via `Shift+S` without re-uploading texture data.
