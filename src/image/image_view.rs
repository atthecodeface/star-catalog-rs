//a Imports
use geo_nd::{Quaternion, Vector, Vector3};
use image::{DynamicImage, GenericImage, Rgba};

use crate::Star;
use crate::{Quat, Vec2, Vec3};

//a ImageView
//tp StarDrawStyle
/// Style for drawing stars
#[derive(Debug, PartialEq, Eq)]
pub enum StarDrawStyle {
    Round,
    Cross,
}

//tp ImageView
/// This is a window onto a [DynamicImage] that allows a sky map to be
/// drawn into it
///
/// Methods are supplied to draw a sky map region to a window of the
/// image, using various controls for field of view, star drawing
/// methods, grids, and so on
///
/// This may be used to generate a single image that is a drawing of
/// what one might see in the sky; it can also be used to generate a
/// cubemap for the whole sky such as might be used in a game or sky
/// viewer
// tan_fov is frame mm width / focal length in mm
pub struct ImageView {
    /// Size of the stars to draw (dimmer stars may be smaller)
    star_size: u32,
    /// Function to use to draw a star
    #[allow(clippy::type_complexity)]
    draw: &'static dyn Fn(&mut ImageView, f64, f64, Rgba<u8>, u32),
    /// Width of the window into the image
    width: u32,
    /// Height of the window into the image
    height: u32,
    /// Offset of the window into the image
    offset: (u32, u32),
    /// Scaling such that tan(angle) maps to half width and half height
    tan_fov_x2: f64,
    /// Orientation of the view
    orient: Quat,
    /// The backing image to be drawn to
    image: DynamicImage,
}

//ip ImageView
impl ImageView {
    //cp new
    /// Create a new image_view
    pub fn new(image: DynamicImage) -> Self {
        let width = image.width();
        let height = image.height();
        let star_size = 10;
        Self {
            star_size,
            draw: &ImageView::draw_round_star,
            width,
            height,
            offset: (0, 0),
            tan_fov_x2: 2.0,
            orient: Quat::unit(),
            image,
        }
    }

    //mp set_tan_hfov
    /// Set the horizontal field of view of the window
    ///
    /// The value supplied must be tan of half of the horizontal field
    /// of view
    pub fn set_tan_hfov(&mut self, tan_hfov: f64) -> &mut Self {
        self.tan_fov_x2 = tan_hfov * 2.0;
        self
    }

    //mp set_orient
    /// Set the orientation of the view
    ///
    /// This expects a unit quaternion which describes the 'camera'
    /// direction and orientation (which way is 'up')
    ///
    /// A common way to generat this is with Quat::look_at(dir, up)
    pub fn set_orient(&mut self, orient: Quat) -> &mut Self {
        self.orient = orient;
        self
    }
    //mp set_window
    /// Set the window to draw in
    pub fn set_window(&mut self, offset: (u32, u32), width: u32, height: u32) -> &mut Self {
        self.offset = offset;
        self.width = width;
        self.height = height;
        self
    }

    //mp set_star_size
    /// Set the size of stars
    pub fn set_star_size(&mut self, star_size: u32) -> &mut Self {
        self.star_size = star_size;
        self
    }

    //mp set_draw_style
    /// Set the style of drawing stars
    pub fn set_draw_style(&mut self, draw_style: StarDrawStyle) -> &mut Self {
        match draw_style {
            StarDrawStyle::Round => {
                self.draw = &ImageView::draw_round_star;
            }
            StarDrawStyle::Cross => {
                self.draw = &ImageView::draw_cross;
            }
        }
        self
    }

    //mi pxy_of_vec
    /// Return a pixel XY of a Vec3 using the current transformation; return None if the pixel would be more than px pixels outside the image window bounds
    ///
    fn pxy_of_vec(&self, v: &Vec3, px: f64) -> Option<Vec2> {
        let v = self.orient.apply3(v);
        if v[2] > 0. {
            return None;
        }
        let tx = -v[0] / v[2];
        let ty = -v[1] / v[2];
        let x = (self.width as f64) * (0.5 + tx / self.tan_fov_x2);
        let y = (self.height as f64) * (0.5 - ty / self.tan_fov_x2);
        if x < -px || x >= self.width as f64 + px {
            return None;
        }
        if y < -px || y >= self.height as f64 + px {
            return None;
        }
        Some([x, y].into())
    }

    //mi put
    /// Set a pixel of the image to an Rgba value
    fn put(&mut self, x: u32, y: u32, color: image::Rgba<u8>) {
        self.image
            .put_pixel(x + self.offset.0, y + self.offset.1, color);
    }

    //mp draw_star
    /// Draw a star at the correct position
    pub fn draw_star(&mut self, s: &Star) {
        /// This is the maximum number of pixels off 'screen' that a
        /// star can be centred while still requiring some pixels to
        /// be drawn
        const MAX_SIZE: f64 = 15.0;
        if let Some(xy) = self.pxy_of_vec(&s.vector, MAX_SIZE) {
            let (r, g, b) = Star::temp_to_rgb(s.temp());
            let color = [
                (r.clamp(0., 1.) * 255.9).floor() as u8,
                (g.clamp(0., 1.) * 255.9).floor() as u8,
                (b.clamp(0., 1.) * 255.9).floor() as u8,
                0,
            ]
            .into();
            let size = ((7.0 - s.mag).clamp(1.0, 6.).powi(2) / 36.0) * self.star_size as f32;
            (self.draw)(self, xy[0], xy[1], color, size as u32);
        }
    }

    //mp draw_right_ascension_lines
    /// Draw a grid with major and minor markings
    pub fn draw_right_ascension_lines(
        &mut self,
        spacing_arcsecs: u32,
        major_arcsecs: u32,
        de_step_arcsecs: u32,
    ) {
        let color_0 = [100, 10, 10, 0].into();
        let color_1 = [10, 100, 10, 0].into();
        let major_spacing = major_arcsecs / spacing_arcsecs;
        let ra_steps_for_180_degrees = 180 * 3600 / spacing_arcsecs;
        for ra_i in 0..2 * ra_steps_for_180_degrees {
            let ra = (ra_i as f64) / (ra_steps_for_180_degrees as f64) * std::f64::consts::PI;
            let color = {
                if ra_i % major_spacing == 0 {
                    color_1
                } else {
                    color_0
                }
            };
            let de_steps_for_180_degrees = 180 * 3600 / de_step_arcsecs;
            for de_i in 0..de_steps_for_180_degrees {
                let de = ((de_i as f64) / (de_steps_for_180_degrees as f64) * 2.0 - 1.0)
                    * std::f64::consts::PI;
                let v = Star::vec_of_ra_de(ra, de);
                if let Some(xy) = self.pxy_of_vec(&v, 0.) {
                    self.put(xy[0] as u32, xy[1] as u32, color);
                }
            }
        }
    }

    pub fn draw_declination_lines(
        &mut self,
        spacing_arcsecs: u32,
        major_arcsecs: u32,
        ra_step_arcsecs: u32,
    ) {
        let color_0 = [100, 10, 10, 0].into();
        let color_1 = [10, 100, 10, 0].into();
        let major_spacing = major_arcsecs / spacing_arcsecs;
        let de_steps_for_180_degrees = 180 * 3600 / spacing_arcsecs;
        for de_i in 0..de_steps_for_180_degrees {
            let de = ((de_i as f64) / (de_steps_for_180_degrees as f64) * 2.0 - 1.0)
                * std::f64::consts::PI;
            let color = {
                if de_i % major_spacing == 0 {
                    color_1
                } else {
                    color_0
                }
            };
            let ra_steps_for_180_degrees = 180 * 60 * 60 / ra_step_arcsecs;
            for ra_i in 0..2 * ra_steps_for_180_degrees {
                let ra = (ra_i as f64) / (ra_steps_for_180_degrees as f64) * std::f64::consts::PI;
                let v = Star::vec_of_ra_de(ra, de);
                if let Some(xy) = self.pxy_of_vec(&v, 0.) {
                    self.put(xy[0] as u32, xy[1] as u32, color);
                }
            }
        }
    }
    //mp draw_grid
    /// Draw a grid with major and minor markings
    pub fn draw_grid(&mut self) {
        self.draw_right_ascension_lines(60 * 60 * 2, 60 * 60 * 10, 6 * 60);
        self.draw_declination_lines(60 * 60 * 5, 60 * 60 * 10, 6 * 60);
    }

    //mp draw_cross
    /// Draw a cross on the image
    fn draw_cross(&mut self, x: f64, y: f64, color: Rgba<u8>, size: u32) {
        // draw a cross
        let x = x as u32;
        let y = y as u32;
        for dx in 0..(2 * size + 1) {
            if x + dx >= size && x + dx - size < self.width {
                self.put(x + dx - size, y, color);
            }
        }
        for dy in 0..(2 * size + 1) {
            if y + dy >= size && y + dy - size < self.height {
                self.put(x, y + dy - size, color);
            }
        }
    }

    //mp draw_round_star
    /// Draw a round star on the image
    fn draw_round_star(&mut self, x: f64, y: f64, color: Rgba<u8>, size: u32) {
        for dx in 0..size + 1 {
            let f_dx = dx as f64;
            let x_p_ib = x + f_dx >= 0. && x + f_dx < self.width as f64;
            let x_m_ib = x - f_dx >= 0. && x - f_dx < self.width as f64;
            if !x_m_ib && !x_p_ib {
                continue;
            }
            for dy in 0..size + 1 {
                if dx * dx + dy * dy > size * size {
                    continue;
                }
                let f_dy = dy as f64;
                let y_p_ib = y + f_dy >= 0. && y + f_dy < self.height as f64;
                let y_m_ib = y - f_dy >= 0. && y - f_dy < self.height as f64;

                if x_p_ib && y_p_ib {
                    self.put((x + f_dx) as u32, (y + f_dy) as u32, color);
                }
                if x_p_ib && y_m_ib {
                    self.put((x + f_dx) as u32, (y - f_dy) as u32, color);
                }
                if x_m_ib && y_p_ib {
                    self.put((x - f_dx) as u32, (y + f_dy) as u32, color);
                }
                if x_m_ib && y_m_ib {
                    self.put((x - f_dx) as u32, (y - f_dy) as u32, color);
                }
            }
        }
    }

    //mp draw_line_between_stars
    /// Draw a line between two stars
    ///
    ///
    pub fn draw_line_between_stars(&mut self, c: Rgba<u8>, s0: &Star, s1: &Star) {
        // Get q = quaternion that maps s0 to [1,0,0], and s1 to [c,s,0]
        let up = s0.vector.cross_product(&s1.vector).normalize();
        let q = Quat::of_axis_angle(&[1., 0., 0.].into(), std::f64::consts::PI / 2.0)
            * Quat::of_axis_angle(&[0., -1., 0.].into(), std::f64::consts::PI / 2.0)
            * Quat::look_at(&s0.vector, &up);
        let angle = s0.cos_angle_between(s1).acos();
        // draw circle needs quat to map [1,0,0] to s0, and [c,s,0] to map to
        self.draw_circle(c, q.conjugate(), angle);
    }

    //mp draw_circle
    /// Draw part of a great circle that is defined applying `quat` to
    /// `[1.,0.,0.]` `[cos(angle), sin(angle), 0.]`.
    pub fn draw_circle(&mut self, c: Rgba<u8>, mut quat: Quat, mut angle: f64) {
        if angle < 1.0E-6 {
            return;
        }
        const MAX_ANGLE_TO_DRAW: f64 = 0.5;
        while angle > MAX_ANGLE_TO_DRAW {
            self.draw_circle(c, quat, MAX_ANGLE_TO_DRAW);
            quat = quat
                * Quat::of_rijk(
                    (MAX_ANGLE_TO_DRAW / 2.0).cos(),
                    0.,
                    0.,
                    (-MAX_ANGLE_TO_DRAW / 2.0).sin(),
                );
            angle -= MAX_ANGLE_TO_DRAW;
        }
        let v0 = quat.apply3(&[1., 0., 0.].into());
        let v1 = quat.apply3(&[angle.cos(), angle.sin(), 0.].into());
        let Some(p0) = self.pxy_of_vec(&v0, self.width as f64) else {
            let Some(p1) = self.pxy_of_vec(&v1, self.width as f64) else {
                return;
            };
            angle = angle / 2.0;
            quat = quat * Quat::of_rijk((angle / 2.0).cos(), 0., 0., (-angle / 2.0).sin());
            return self.draw_circle(c, quat, angle);
        };
        let Some(p1) = self.pxy_of_vec(&v1, self.width as f64) else {
            angle = angle / 2.0;
            return self.draw_circle(c, quat, angle);
        };
        let m = quat.apply3(&[(angle / 2.0).cos(), (angle / 2.0).sin(), 0.].into());
        let Some(pm) = self.pxy_of_vec(&m, self.width as f64) else {
            return;
        };
        if (p1 - pm).normalize().dot(&(pm - p0).normalize()) > 0.999 {
            let dp = (p1 - p0) / 20.0;
            for i in 0..20 {
                let f = i as f64;
                let p = p0 + dp * f;
                if p[0] < 0.0 || p[0] >= self.width as f64 {
                    continue;
                }
                if p[1] < 0.0 || p[1] >= self.height as f64 {
                    continue;
                }
                self.put(p[0] as u32, p[1] as u32, c);
            }
        } else {
            angle = angle / 2.0;
            self.draw_circle(c, quat, angle);
            quat = quat * Quat::of_rijk((angle / 2.0).cos(), 0., 0., (-angle / 2.0).sin());
            return self.draw_circle(c, quat, angle);
        }
    }

    //mp take_image
    /// Drop the [ImageView] and take the image, so it can be saved
    pub fn take_image(self) -> DynamicImage {
        self.image
    }
}
