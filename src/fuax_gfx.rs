use sdl2::render::{WindowCanvas, BlendMode};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub trait FauxGFX {
	fn draw_quadrants(&mut self, p: Point, dp: Point, f: i32) -> Result<(), String>;
	fn pixel_rgba_weight(&mut self, p: Point, color: Color, weight: i32) -> Result<(), String>;
	fn pixel_rgba(&mut self, p: Point, color: Color) -> Result<(), String>;
	fn pixel(&mut self, p: Point) -> Result<(), String>;
	fn vline(&mut self, x: i32, y1: i32, y2: i32) -> Result<(), String>;
	fn vline_rgba(&mut self, x: i32, y1: i32, y2: i32, color: Color) -> Result<(), String>;
	fn hline(&mut self, x1: i32, x2: i32, y: i32) -> Result<(), String>;
	fn hline_rgba(&mut self, x1: i32, x2: i32, y: i32, color: Color) -> Result<(), String>;
	fn ellipse_rgba(&mut self, p: Point, radius_x: i16, radius_y: i16, color: Color, f: i32) -> Result<(), String>;
	fn filled_circle(&mut self, p: Point, rad: i16, color: Color) -> Result<(), String>;
	fn filled_polygon_rgba_mt(&mut self, verts: Vec<Point>, color: Color) -> Result<(), String>;
	fn thick_line(&mut self, start: Point, end: Point, width: i32, color: Color)-> Result<(), String>;
	fn box_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color) -> Result<(), String>;
	fn line_rgba(&mut self, start: Point, end: Point, color: Color) -> Result<(), String>;
	fn rectangle_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color) -> Result<(), String>;
	fn rounded_rentangle_rgba(&mut self, top_right: Point, bottom_left: Point, radius: i32, color: Color)-> Result<(), String>;
	fn arc_rgba(&mut self,p: Point, radius: i32, start: i32, end: i32, color: Color) -> Result<(), String>;
	fn rounded_box_rgba(&mut self, top_right: Point, bottom_left: Point, radius: i32, color: Color) -> Result<(), String>;
}

impl FauxGFX for WindowCanvas {

	fn line_rgba(&mut self, start: Point, end: Point, color: Color) -> Result<(), String> {
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.draw_line(start, end);
	}
	
	fn hline_rgba(&mut self, x1: i32, x2: i32, y: i32, color: Color) -> Result<(), String> {
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.draw_line(Point::new(x1, y), Point::new(x2, y));
	}
	
	fn vline_rgba(&mut self, x: i32, y1: i32, y2: i32, color: Color ) -> Result<(), String> {
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.draw_line(Point::new(x, y1), Point::new(x, y2));
	}
	
	fn box_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color) -> Result<(), String> {
		if top_right.x == bottom_left.x {
			if top_right.y == bottom_left.y {
				return self.pixel_rgba(top_right, color);
			} else {
				return self.vline_rgba(top_right.x, top_right.y, bottom_left.y , color);	
			}
		} else if top_right.y == bottom_left.y {
			return self.hline_rgba(top_right.x, bottom_left.x, top_right.y, color);
		}
		

		let mut x_1 = top_right.x;
		let mut x_2 = bottom_left.x;
		let mut y_1 = top_right.y;
		let mut y_2 = bottom_left.y;

		if x_1 > x_2 {
			let tmp = x_1;
			x_1 = x_2;
			x_2 = tmp;
		}

		if y_1 > y_2 {
			let tmp = y_1;
			y_1 = y_2;
			y_2 = tmp;
		}


		let rect = Rect::new(x_1, y_1, (x_2 - x_1 + 1) as u32, (y_2 - y_1 + 1) as u32);

		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.fill_rect(rect);
	}

	fn thick_line(&mut self, start: Point, end: Point, width: i32, color: Color) -> Result<(), String> {
		if width < 1 {
			return Err(String::from("Width not valid for line"));
		}

		if start.x == end.x && start.y == start.y {
			let wh = width / 2;
			return self.box_rgba(Point::new(start.x - wh, start.y - wh), Point::new(end.x + width, end.y + width), color);
		}

		if width == 1 {
			return self.line_rgba(start, end, color);
		}

		let dx = (end.x - start.x) as f32;
		let dy = (end.y - start.y) as f32;
		let l = (dx * dx + dy * dy).sqrt();
		let ang = dx.atan2(dy);
		let adj = 0.1 + 0.9 * (2.0 * ang).cos().abs();
		let wl2  = ((width as f32) - adj) / (2.0 * l);
		let nx = dx * wl2;
		let ny = dy * wl2;

		let dx1 = start.x as f32;
		let dy1 = start.y as f32;
		let dx2 = end.x as f32;
		let dy2 = end.y as f32; 
  

		let verts = vec![Point::new((dx1 + ny) as i32, (dy1 - nx)  as i32),
							 Point::new((dx1 - ny) as i32, (dy1 + nx)  as i32),
							 Point::new((dx2 - ny) as i32, (dy2 + nx)  as i32),
							 Point::new((dx2 + ny) as i32, (dy2 - nx)  as i32)];

		return self.filled_polygon_rgba_mt(verts, color);
	}

	fn filled_polygon_rgba_mt(&mut self, verts: Vec<Point>, color: Color) -> Result<(), String> {
		if verts.len() < 3  {
			return Err(String::from("Not enough vertices"));
		}


		let mut polygon_indices : Vec<i32> = vec![0; verts.len()];

		let mut min_y = verts[0].y;
		let mut max_y = verts[0].y;

		for vert in &verts {
			if vert.y < min_y {
				min_y = vert.y;
			}
			else if vert.y > max_y {
				max_y = vert.y;
			}
		}
		let mut indices_1;
		let mut indices_2;

		let mut y_1;
		let mut y_2;

		let mut x_1;
		let mut x_2;
		for y in min_y..=max_y {
			let mut ints = 0;
			for i in 0..verts.len() {
				if i == 0 {
					indices_1 = verts.len() - 1;
					indices_2 = 0;
				}
				else {
					indices_1 = i - 1;
					indices_2 = i;
				}
				y_1 = verts[indices_1].y;
				y_2 = verts[indices_2].y;
				if y_1 < y_2 {
					x_1 = verts[indices_1].x;
					x_2 = verts[indices_2].x;
				}
				else if y_1 > y_2 {
					y_2 = verts[indices_1].y;
					y_1 = verts[indices_2].y;
					x_2 = verts[indices_1].x;
					x_1 = verts[indices_2].x;
				}
				else {
					continue;
				}

				if ((y >= y_1) && (y < y_2)) || ((y == max_y) && (y > y_1) && (y <= y_2)) {
					polygon_indices[ints] = ((65536 * (y - y_1)) / (y_2 - y_1)) * (x_2 - x_1) + (65536 * x_1);
					ints += 1;
				}
			}



			polygon_indices.sort_unstable();

			let blend_mode : BlendMode;
			if color.a == 255 {
				blend_mode = BlendMode::None;
			}
			else {
				blend_mode = BlendMode::Blend;
			}

			self.set_blend_mode(blend_mode);
			self.set_draw_color(color);

			let mut i = 0;
			
			let mut x_a;
			let mut x_b;

			while i < ints {
				x_a = polygon_indices[i] + 1;
				x_a = (x_a >> 16) + ((x_a & 32768) >> 15);	

				x_b = polygon_indices[i + 1] - 1;
				x_b = (x_b >> 16) + ((x_b & 32768) >> 15);	         
				i += 2;

				let result = self.hline(x_a, x_b, y);
				if result.is_err() {
					return result;
				}
			}
		}

		return Ok(());
	}

	fn draw_quadrants(&mut self, p: Point, dp: Point, f: i32) -> Result<(), String> {
		if dp.x == 0 {
			if dp.y == 0 {
				return self.pixel(p);
			} else {
				let ypdy = p.y + dp.y;
				let ymdy = p.y - dp.y;

				if f != 0  {
					return self.vline(p.x, ymdy, ypdy);
				}
				else {
					return self.pixel(Point::new(p.x, ypdy)).and_then(|_| {
						return self.pixel(Point::new(p.x, ymdy));
					});
				}
			}
		} else {
			let xpdx = p.x + dp.x;
			let xmdx = p.x - dp.x;
			let ypdy = p.y + dp.y;
			let ymdy = p.y - dp.y;

			if f != 0 {
				return self.vline(xpdx, ymdy, ypdy).and_then(|_|{
					return self.vline(xmdx, ymdy, ypdy);
				});
			} else {
				return self.pixel(Point::new(xpdx, ypdy)).and_then(|_|{
					return self.pixel(Point::new(xmdx, ypdy)).and_then(|_|{
						return self.pixel(Point::new(xpdx, ymdy)).and_then(|_| {
							return self.pixel(Point::new(xmdx, ymdy));
							});
						});
				});
			}
		}
	}

	fn pixel_rgba_weight(&mut self, p: Point, color: Color, weight: i32) -> Result<(), String> {

		let mut a_x = color.a as i32;
		a_x = (a_x * weight) >> 8;
		if a_x > 255 {
			a_x = 255;
		} else {
			a_x = a_x & 0x000000ff;
		}
		let new_color = Color::RGBA(color.r, color.g, color.b, a_x as u8);

		return self.pixel_rgba(p, new_color);
	}

	fn pixel_rgba(&mut self, p: Point, color: Color) -> Result<(), String> {
		let blend_mode : BlendMode;

		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.draw_point(p);
	}

	fn pixel(&mut self, p: Point) -> Result<(), String> {
		return self.draw_point(p);
	}

	fn vline(&mut self, x: i32, y1: i32, y2: i32) -> Result<(), String> {
		return self.draw_line(Point::new(x , y1), Point::new(x, y2));
	}

	fn hline(&mut self, x1: i32, x2: i32, y: i32) -> Result<(), String> {
		return self.draw_line(Point::new(x1, y), Point::new(x2, y));
	}

	fn ellipse_rgba(&mut self, p: Point, radius_x: i16, radius_y: i16, color: Color, f: i32) -> Result<(), String> {

	//   println!("{} {}", radius_x, radius_y);
		if radius_x < 0 || radius_y < 0 {
			return Err(String::from(""));
		}
		let  default_eclipse_overscan =	4;

		let blend_mode : BlendMode;

		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);

		if radius_x == 0 {
			if radius_y == 0 {
				return self.pixel(p);
			} else {
				return self.vline(p.x, p.y - radius_y as i32, p.y + radius_y as i32);
			}
		} else {
			if radius_y == 0 {
				return self.hline(p.x - radius_x as i32, p.x + radius_x as i32, p.y);
			}
		}


		let mut rxi : i32 = radius_x as i32;
		let mut ryi : i32 = radius_y as i32;

		let ellipse_overscan;

		if rxi >= 512 || ryi >= 512 {
			ellipse_overscan = default_eclipse_overscan  / 4;
		}
		else if rxi >= 256 || ryi >= 256 {
			ellipse_overscan = default_eclipse_overscan / 2;
		}
		else {
			ellipse_overscan = default_eclipse_overscan / 1;
		}

		let mut old_x : i32  = 0;
		let mut old_y : i32 = 0;
		let mut src_x : i32 = 0;
		let mut src_y : i32;
		let result = self.draw_quadrants(p, Point::new(0, radius_y as i32), f);
		if result.is_err() {
			return result;
		}
		rxi *= ellipse_overscan;
		ryi *= ellipse_overscan;
		let rx2 : i32 = rxi * rxi;
		let rx22 : i32 = rx2 + rx2;
		let ry2 : i32 = ryi * ryi;
		let ry22 : i32 = ry2 + ry2;
		let mut curX : i32 = 0;
		let mut curY : i32 = ryi;
		let mut deltaX : i32 = 0;
		let mut deltaY : i32 = rx22 * curY;

		let mut error : i32 = ry2 - rx2 * ryi + rx2 / 4;
		while deltaX <= deltaY
		{
			curX+=1;
			deltaX += ry22;
	
			error +=  deltaX + ry2; 
			if error >= 0
			{
				curY-=1;
				deltaY -= rx22; 
				error -= deltaY;
			}

			src_x = curX / ellipse_overscan;
			src_y = curY / ellipse_overscan;
			if (src_x != old_x && src_y == old_y) || (src_x != old_x && src_y != old_y) {
				let result = self.draw_quadrants(p, Point::new(src_x as i32, src_y as i32), f);
				if result.is_err() {
					return result;
				}
				old_x = src_x;
				old_y = src_y;
			}
		}

		if curY > 0 
		{
			let curXp1 = curX + 1;
			let curYm1 = curY - 1;


			//As best as I an tell SDL_GFX is using the under/over flow behavior of the these varibles to figure out what to do
			let inner = ry2 + 3;
			let inner_2 = inner / 4;
			let head = curX.wrapping_mul(curXp1);
			let head_head = ry2.wrapping_mul(head);
			let head_inner = head_head.wrapping_add(inner_2);
			let first_mul = rx2.wrapping_mul(curYm1);
			let second_mul = first_mul.wrapping_mul(curYm1);
			let final_mul = rx2.wrapping_mul(ry2);
			error = head_inner.wrapping_add(second_mul.wrapping_sub(final_mul));

			while curY > 0
			{
				curY-=1;
				deltaY -= rx22;

				error += rx2;
				error -= deltaY;
	
				if error <= 0 
				{
				curX+=1;
				deltaX += ry22;
				error += deltaX;
				}

				src_x = curX / ellipse_overscan;
				src_y = curY / ellipse_overscan;
				if (src_x != old_x && src_y == old_y) || (src_x != old_x && src_y != old_y) {
					old_y-=1;
					while old_y >= src_y {
						self.draw_quadrants(p , Point::new(src_x, old_y), f)?;
						if f != 0 {
							old_y = src_y - 1;
						}
						old_y -=1;
					}
					old_x = src_x;
					old_y = src_y;
				}		
			}

			/* Remaining points in vertical */
			if f != 0 {
				old_y-=1;

				for i in old_y..=0 {
					self.draw_quadrants(p , Point::new(src_x,  i), f)?;
				}
			}
		}

		return Ok(());
	}


	fn filled_circle(&mut self, p: Point, rad: i16, color: Color) -> Result<(), String> {
		return self.ellipse_rgba(p, rad, rad, color, 1);
	}

	fn rectangle_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color) -> Result<(), String> {

		if top_right.x == bottom_left.x {
			if top_right.y == bottom_left.y {
				return self.pixel_rgba(top_right, color);
			}
			else {
				return self.vline_rgba(top_right.x, top_right.y, bottom_left.y, color);
			}
		}
		else if top_right.y == bottom_left.y {
			return self.hline_rgba(top_right.x, bottom_left.x, top_right.y, color);
		} 	

		let mut x_1 = top_right.x;
		let mut x_2 = bottom_left.x;
		let mut y_1 = top_right.y;
		let mut y_2 = bottom_left.y;

		if x_1 > x_2 {
			let tmp = x_1;
			x_1 = x_2;
			x_2 = tmp;
		}

		if y_1 > y_2 {
			let tmp = y_1;
			y_1 = y_2;
			y_2 = tmp;
		}


		let rect = Rect::new(x_1, y_1, (x_2 - x_1 + 1) as u32, (y_2 - y_1 + 1) as u32);

		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		return self.fill_rect(rect);
	}

	fn rounded_rentangle_rgba(&mut self, top_right: Point, bottom_left: Point, radius: i32, color: Color) -> Result<(), String> {
		if radius < 0 {
			return Err(String::from("less then 0 radius not allowed"));
		}

		if radius <= 1 {
			return self.rectangle_rgba(top_right, bottom_left, color);
		}

		if top_right.x == bottom_left.x {
			if top_right.y == bottom_left.y {
				return self.pixel_rgba(top_right, color);
			}
			else {
				return self.vline_rgba(top_right.x, top_right.y, bottom_left.y, color);
			}
		}
		else if top_right.y == bottom_left.y {
			return self.hline_rgba(top_right.x, bottom_left.x, top_right.y, color);
		} 	

		let mut x_1 = top_right.x;
		let mut x_2 = bottom_left.x;
		let mut y_1 = top_right.y;
		let mut y_2 = bottom_left.y;

		if x_1 > x_2 {
			let tmp = x_1;
			x_1 = x_2;
			x_2 = tmp;
		}

		if y_1 > y_2 {
			let tmp = y_1;
			y_1 = y_2;
			y_2 = tmp;
		}

		let mut rad = radius;
		let mut w = bottom_left.x - top_right.x;
		let mut h = bottom_left.y - top_right.y;

		if rad * 2 > w {
			rad = w / 2;
		}

		if rad * w > h {
			rad = h / 2;
		}

		let xx1 = x_1 + rad;
		let xx2 = x_2 - rad;
		let yy1 = y_1 + rad;
		let yy2 = y_2 - rad;
		self.arc_rgba(Point::new(xx1, yy1), rad, 180, 270, color);
		self.arc_rgba(Point::new(xx2, yy1), rad, 270, 360, color);
		self.arc_rgba(Point::new(xx1, yy2), rad,  90, 180, color);
		self.arc_rgba(Point::new(xx2, yy2), rad,   0,  90, color);

	if xx1 <= xx2 {
		self.hline_rgba(xx1, xx2, y_1, color);
		self.hline_rgba(xx1, xx2, y_2, color);
	}
	if yy1 <= yy2 {
		self.vline_rgba(x_1, yy1, yy2, color);
		self.vline_rgba(x_2, yy1, yy2, color);
	}


		return Ok(());
	}

	fn arc_rgba(&mut self,p: Point, radius: i32, start: i32, end: i32, color: Color) -> Result<(), String>{

		if radius < 0 {
			return Err(String::from("Cannot draw arc with radius les then 0"));
		}
		let mut cy = radius;
		let mut df = 1 - radius;
		let mut d_e = 3;
		let mut d_se = -2 * radius + 5;

		if radius == 0 {
			return self.pixel_rgba(p, color);
		}

		let mut drawoct = 0;

		let mut use_start = start % 360;
		let mut use_end = end % 360;

		while use_start < 0 {
			use_start += 360;
		}

		while use_end < 0 {
			use_end += 360;
		}

		use_start %= 360;
		use_end %= 360;

		let start_oct = use_start / 45;
		let end_oct = use_end / 45;
		let mut oct = start_oct - 1;
		let mut dstart : f32;
		let mut dend : f32;
		let mut temp : f32 = 0.0;
		let mut stopval_start : i32 = 0;
		let mut stopval_end : i32 = 0;


		//this is replicate the do while loop 
		//of the original source code		
		let mut once = true;

		while once || oct != end_oct {
			once = false;
			oct = (oct + 1) % 8;

			if oct == start_oct {
				dstart = use_start as f32;
				match oct {
					0 | 3 => {
						temp = (dstart * std::f32::consts::PI / 180.0).sin()
					},
					1 | 6 => {
						temp = (dstart * std::f32::consts::PI / 180.0).cos()
					},
					2 | 5 => {
						temp = -(dstart * std::f32::consts::PI / 180.0).cos()
					},
					4 | 7 => {
						temp = -(dstart * std::f32::consts::PI / 180.0).sin()
					} 
					_ => {

					}
				}

				temp = temp * radius as f32;
				stopval_start = temp as i32;
				if oct % 2 != 0 {
					drawoct |= 1 << oct;
				} else {
					drawoct &= 255 - (1 << oct);
				}
			}
			if oct == end_oct {
				dend = use_end as f32;
				match oct {
					0 | 3 => {
						temp = (dend * std::f32::consts::PI / 180.0).sin()
					},
					1 | 6 => {
						temp = (dend * std::f32::consts::PI / 180.0).cos()
					},
					2 | 5 => {
						temp = -(dend * std::f32::consts::PI / 180.0).cos()
					},
					4 | 7 => {
						temp = -(dend * std::f32::consts::PI / 180.0).sin()
					} 
					_ => {

					}
				}

				temp = temp * radius as f32;
				stopval_end = temp as i32;

				if start_oct == end_oct {
					if use_start > use_end {
						drawoct = 255;
					} else {
						drawoct &= 255 - (1 << oct);
					}
				}
				else if oct % 2 != 0 {
					drawoct &= 255 - (1 << oct);
				} else {
					drawoct |= 1 << oct;
				}
			} else if start_oct != oct {
				drawoct |= 1 << end_oct;
			}
		}

		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);

		let mut once = true;

		let mut cx = 0;

		let mut ypcy;
		let mut ymcy;
		let mut xpcx;
		let mut xmcx;
		let mut xmcy;
		let mut	xpcy;
		let mut	ypcx;
		let mut ymcx;

		while once || cx <= cy {
			once = false;
			ypcy = p.y + cy;
			ymcy = p.y - cy;

			if cx > 0 {
				xpcx = p.x + cx;
				xmcx = p.x - cx;
				if drawoct & 4 != 0 {
					self.pixel(Point::new(xmcx, ypcy))?;
				} else if drawoct & 2 != 0 {
					self.pixel(Point::new(xpcx, ypcy))?;
				} else if drawoct & 32 != 0 {
					self.pixel(Point::new(xmcx, ymcy))?;
				} else if drawoct & 64 != 0 {
					self.pixel(Point::new(xpcx, ymcy))?;
				}

			} else {
				if drawoct & 96 != 0 {
					self.pixel(Point::new(p.x, ymcy))?;
				} else if drawoct & 6 != 0 {
					self.pixel(Point::new(p.x, ypcy))?;
				}
			}

			xpcy = p.x + cy;
			xmcy = p.x - cy;

			if cx > 0 && cx != cy {
				ypcx = p.y + cx;
				ymcx = p.y - cx;

				if drawoct & 8 != 0 {
					self.pixel(Point::new(xmcy, ypcx))?;
				} else if drawoct & 1 != 0 {
					self.pixel(Point::new(xpcy, ypcx))?;
				} else if drawoct & 16 != 0 {
					self.pixel(Point::new(xmcy, ymcx))?;
				} else if drawoct & 128 != 0 {
					self.pixel(Point::new(xpcy, ymcx))?;
				}
			} else if cx == 0 {
				if drawoct & 24 != 0 {
					self.pixel(Point::new(xmcy, p.y))?;
				} else if drawoct & 129 != 0 {
					self.pixel(Point::new(xpcy, p.y))?;
				}
			}

			if stopval_start == cx {
				if drawoct & (1 << start_oct) != 0 {
					drawoct &= 255 - (1 << start_oct);
				} else {
					drawoct |= 1 << start_oct;
				}
			}
			if stopval_end == cx {
				if drawoct & (1 << end_oct) != 0 {
					drawoct &= 255 - (1 << end_oct);
				} else {
					drawoct |= 1 << end_oct;
				}
			}


			if df < 0 {
				df += d_e;
				d_e += 2;
				d_se +=2 ;
			} else {
				df += d_se;
				d_e += 2;
				d_se += 4;
				cy -= 1;
			}
			cx += 1;
		}


		return Ok(());
	}

	fn rounded_box_rgba(&mut self, top_right: Point, bottom_left: Point, radius: i32, color: Color) -> Result<(), String>{
			
		if radius < 0 {
			return Err(String::from("Must have a radius larger then 0"));
		}

		if radius <= 1 {
			return self.box_rgba(top_right, bottom_left, color);
		}


		if top_right.x == bottom_left.x {
			if top_right.y == bottom_left.y {
				return self.pixel_rgba(top_right, color);
			}
			else {
				return self.vline_rgba(top_right.x, top_right.y, bottom_left.y, color);
			}
		}
		else if top_right.y == bottom_left.y {
			return self.hline_rgba(top_right.x, bottom_left.x, top_right.y, color);
		} 	

		let mut x_1 = top_right.x;
		let mut x_2 = bottom_left.x;
		let mut y_1 = top_right.y;
		let mut y_2 = bottom_left.y;

		if x_1 > x_2 {
			let tmp = x_1;
			x_1 = x_2;
			x_2 = tmp;
		}

		if y_1 > y_2 {
			let tmp = y_1;
			y_1 = y_2;
			y_2 = tmp;
		}

		let mut rad;
		let w = bottom_left.x - top_right.x;
		let h = bottom_left.y - top_right.y;

		let r_2 = radius + radius;

		if r_2 > w {
			rad = w / 2;
		}

		if r_2 > h {
			rad = h / 2;
		}


		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);


		let mut once = false;

		let mut cx = 0;
		let mut cy = radius;

		let mut ocx = 0xffff;
		let mut ocy = 0xffff;
		let mut df = 1 - radius;
		let mut d_e = 3;
		let mut d_se = -2 * radius + 5;

		let mut xpcx;
		let mut xmcx;
		let mut xpcy;
		let mut xmcy;
		let mut ypcy;
		let mut ymcy;
		let mut ypcx;
		let mut ymcx;
		let mut x = top_right.x + radius;
		let mut y = top_right.y + radius;
		let mut dx = bottom_left.x - top_right.x - radius - radius;
		let mut dy = bottom_left.y - top_right.y - radius - radius;


		while once == false && cx <= cy {
			once = true;

			xpcx = top_right.x + cx;
			xmcx = x - cx;
			xpcy = x + cy;
			xmcy = x - cy;
			if ocy != cy {
				if cy > 0 {
					ypcy = y + cy;
					ymcy = y - cy;
					self.hline(xmcx, xpcx + dx, ypcy + dy);
					self.hline(xmcx, xpcx + dx, ymcy);
				} else {
					self.hline(xmcx, xpcx + dx, y);
				}
				ocy = cy;
			}
			if ocx != cx {
				if cx != cy {
					if cx > 0 {
						ypcx = y + cx;
						ymcx = y - cx;
						self.hline(xmcy, xpcy + dx, ymcx);
						self.hline(xmcy, xpcy + dx, ypcx + dy);
					} else {
						self.hline(xmcy, xpcy + dx, y);
					}
				}
				ocx = cx;
			}

			if df < 0 {
				df += d_e;
				d_e += 2;
				d_se += 2;
			} else {
				df += d_se;
				d_e += 2;
				d_se += 4;
				cy-=1;
			}
			cx+=1;
		}


		Ok(())
	}
}