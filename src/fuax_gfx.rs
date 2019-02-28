use sdl2::render::{WindowCanvas, BlendMode};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub trait FauxGFX {
	fn draw_quadrants(&mut self, p: Point, dp: Point, f: i32) -> Result<(), String>;
	fn pixel_rgba(&mut self, p: Point, color: Color) -> Result<(), String>;
	fn pixel(&mut self, p: Point) -> Result<(), String>;
	fn vline(&mut self, x: i32, y1: i32, y2: i32) -> Result<(), String>;
	fn vline_rgba(&mut self, x: i32, y1: i32, y2: i32, color: Color);
	fn hline(&mut self, x1: i32, x2: i32, y: i32) -> Result<(), String>;
	fn hline_rgba(&mut self, x1: i32, x2: i32, y: i32, color: Color);
	fn ellipse_rgba(&mut self, p: Point, radius_x: i16, radius_y: i16, color: Color, f: i32) -> Result<(), String>;
	fn filled_circle(&mut self, p: Point, rad: i16, color: Color);
	fn filled_polygon_rgba_mt(&mut self, verts: Vec<Point>, color: Color);
	fn thick_line(&mut self, start: Point, end: Point, width: i32, color: Color);
	fn box_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color);
	fn line_rgba(&mut self, start: Point, end: Point, color: Color);

}

impl FauxGFX for WindowCanvas {

	fn line_rgba(&mut self, start: Point, end: Point, color: Color){
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		self.draw_line(start, end);
	}
	
	fn hline_rgba(&mut self, x1: i32, x2: i32, y: i32, color: Color){
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		self.draw_line(Point::new(x1, y), Point::new(x2, y));
	}
	
	fn vline_rgba(&mut self, x: i32, y1: i32, y2: i32, color: Color ){
		let blend_mode : BlendMode;
		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}
		self.set_blend_mode(blend_mode);
		self.set_draw_color(color);
		self.draw_line(Point::new(x, y1), Point::new(x, y2));
	}
	
	fn box_rgba(&mut self, top_right: Point, bottom_left: Point, color: Color){
		if top_right.x == bottom_left.x {
			if top_right.y == bottom_left.y {
				self.pixel_rgba(top_right, color);
			} else {
				self.vline_rgba(top_right.x, top_right.y, bottom_left.y , color);	
			}
		} else {
			self.hline_rgba(top_right.x, bottom_left.x, top_right.y, color);
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
		self.fill_rect(rect);
	}

	fn thick_line(&mut self, start: Point, end: Point, width: i32, color: Color) {
		if width < 1 {
			//return ERR
		}

		if start.x == end.x && start.y == start.y {
			let wh = width / 2;
			self.box_rgba(Point::new(start.x - wh, start.y - wh), Point::new(end.x + width, end.y + width), color);
		}

		if width == 1 {
			self.line_rgba(start, end, color);
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

		self.filled_polygon_rgba_mt(verts, color);
	}

	fn filled_polygon_rgba_mt(&mut self, verts: Vec<Point>, color: Color) {
		if verts.len() < 3  {
			//return ERRs
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
			
			let mut xa = 0;
			let mut xb = 0;

			while i < ints {
				xa = polygon_indices[i] + 1;
				xa = (xa >> 16) + ((xa & 32768) >> 15);	

				xb = polygon_indices[i + 1] - 1;
				xb = (xb >> 16) + ((xb & 32768) >> 15);	         
				i += 2;

				self.hline(xa, xb, y);
			}

		}
	}

	fn draw_quadrants(&mut self, p: Point, dp: Point, f: i32) -> Result<(), String> {
		if dp.x == 0 {
			if dp.y == 0 {
				self.pixel(p);
			} else {
				let ypdy = p.y + dp.y;
				let ymdy = p.y - dp.y;

				if f != 0  {
					self.vline(p.x, ymdy, ypdy);
				}
				else {
					self.pixel(Point::new(p.x, ypdy));
					self.pixel(Point::new(p.x, ymdy));
				}
			}
		} else {
			let xpdx = p.x + dp.x;
			let xmdx = p.x - dp.x;
			let ypdy = p.y + dp.y;
			let ymdy = p.y - dp.y;

			if f != 0 {
				self.vline(xpdx, ymdy, ypdy);
				self.vline(xmdx, ymdy, ypdy);
			} else {
				self.pixel(Point::new(xpdx, ypdy));
				self.pixel(Point::new(xmdx, ypdy));
				self.pixel(Point::new(xpdx, ymdy));
				self.pixel(Point::new(xmdx, ymdy));
			}
		}

    	return Ok(())
	}

	fn pixel_rgba(&mut self, p: Point, color: Color) -> Result<(), String> {
		let blend_mode : BlendMode;

		if color.a == 255 {
			blend_mode = BlendMode::None;
		}
		else {
			blend_mode = BlendMode::Blend;
		}

		let _ = self.set_blend_mode(blend_mode);
		let _ = self.set_draw_color(color);
		let _ = self.draw_point(p);

		return Ok(());
	}

	fn pixel(&mut self, p: Point) -> Result<(), String> {
		let _ = self.draw_point(p);
		return Ok(());
	}

	fn vline(&mut self, x: i32, y1: i32, y2: i32) -> Result<(), String> {
		let _ = self.draw_line(Point::new(x , y1), Point::new(x, y2));
		Ok(())
	}

	fn hline(&mut self, x1: i32, x2: i32, y: i32) -> Result<(), String> {
		let _ = self.draw_line(Point::new(x1, y), Point::new(x2, y));
		Ok(())
	}

	fn ellipse_rgba(&mut self, p: Point, radius_x: i16, radius_y: i16, color: Color, f: i32) -> Result<(), String> {

	//   println!("{} {}", radius_x, radius_y);
		if radius_x < 0 || radius_y < 0 {
			return Err(String::from(""));
		}
		let  DEFAULT_ELLIPSE_OVERSCAN =	4;

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

		let ellipseOverscan;

		if rxi >= 512 || ryi >= 512 {
			ellipseOverscan = DEFAULT_ELLIPSE_OVERSCAN  / 4;
		}
		else if rxi >= 256 || ryi >= 256 {
			ellipseOverscan = DEFAULT_ELLIPSE_OVERSCAN / 2;
		}
		else {
			ellipseOverscan = DEFAULT_ELLIPSE_OVERSCAN / 1;
		}

		let mut oldX : i32  = 0;
		let mut oldY : i32 = 0;
		let mut scrX : i32 = 0;
		let mut scrY : i32 = 0;
		self.draw_quadrants(p, Point::new(0, radius_y as i32), f);

		rxi *= ellipseOverscan;
		ryi *= ellipseOverscan;
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

			scrX = curX / ellipseOverscan;
			scrY = curY / ellipseOverscan;
			if (scrX != oldX && scrY == oldY) || (scrX != oldX && scrY != oldY) {
				self.draw_quadrants(p, Point::new(scrX as i32, scrY as i32), f);
				oldX = scrX;
				oldY = scrY;
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

				scrX = curX / ellipseOverscan;
				scrY = curY / ellipseOverscan;
				if (scrX != oldX && scrY == oldY) || (scrX != oldX && scrY != oldY) {
					oldY-=1;
					while oldY >= scrY {
						self.draw_quadrants(p , Point::new(scrX, oldY), f);
						/* revent overdraw */
						if f != 0 {
							oldY = scrY - 1;
						}
						oldY -=1;
					}
					oldX = scrX;
					oldY = scrY;
				}		
			}

			/* Remaining points in vertical */
			if f != 0 {
				oldY-=1;

				for i in oldY..=0 {
					self.draw_quadrants(p , Point::new(scrX,  i), f);
				}
			}
		}

		return Ok(());
	}


	fn filled_circle(&mut self, p: Point, rad: i16, color: Color) {
		self.ellipse_rgba(p, rad, rad, color, 1);
	}

}