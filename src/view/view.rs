use crate::constants::{
	MINIMUM_COMPACT_WINDOW_WIDTH,
	MINIMUM_WINDOW_HEIGHT,
	TITLE,
	TITLE_HELP_INDICATOR_LENGTH,
	TITLE_LENGTH,
	TITLE_SHORT,
	TITLE_SHORT_LENGTH,
};
use crate::display::{Display, DisplayColor};
use crate::scroll::get_scroll_position;
use crate::view::ViewLine;
use crate::Config;

pub struct View<'v> {
	config: &'v Config,
	display: &'v Display<'v>,
}

impl<'v> View<'v> {
	pub fn new(display: &'v Display, config: &'v Config) -> Self {
		Self { display, config }
	}

	pub fn draw_str(&self, s: &str) {
		self.display.draw_str(s);
	}

	pub fn set_color(&self, color: DisplayColor, selected: bool) {
		self.display.color(color, selected);
	}

	pub fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.display.set_style(dim, underline, reverse);
	}

	pub fn check_window_size(&self) -> bool {
		let (window_width, window_height) = self.get_view_size();
		!(window_width <= MINIMUM_COMPACT_WINDOW_WIDTH || window_height <= MINIMUM_WINDOW_HEIGHT)
	}

	pub fn draw_error(&self, message: &str) {
		self.draw_title(false);
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, false, false);
		self.display.draw_str(message);
		self.display.draw_str("\n");
		self.display.color(DisplayColor::IndicatorColor, false);
		self.display.draw_str("Press any key to continue");
	}

	pub fn clear(&self) {
		self.display.clear();
	}

	pub fn get_view_size(&self) -> (usize, usize) {
		let (view_width, view_height) = self.display.get_window_size();
		(view_width as usize, view_height as usize)
	}

	pub fn refresh(&self) {
		self.display.refresh();
	}

	pub fn draw_view_lines(&self, lines: &[ViewLine], top: usize, left: usize, height: usize) {
		let number_of_lines = lines.len();

		let scroll_indicator_index = get_scroll_position(top, number_of_lines, height);
		let show_scroll_bar = height < number_of_lines;

		let mut index: usize = 0;
		for line in lines.iter().skip(top).take(height) {
			self.draw_view_line(line, left, show_scroll_bar);
			if show_scroll_bar {
				self.display.color(DisplayColor::Normal, false);
				self.display.set_style(scroll_indicator_index != index, false, true);
				self.display.draw_str(" ");
			}
			index += 1;
		}

		if height > lines.len() {
			self.draw_vertical_spacer((height - index) as i32);
		}
	}

	pub fn draw_view_line(&self, line: &ViewLine, left: usize, scrollbar: bool) {
		let (window_width, _) = self.display.get_window_size();
		let window_width = if scrollbar { window_width - 1 } else { window_width } as usize;

		let mut start = 0;
		let mut left_start = 0;
		for (i, segment) in line.get_segments().iter().enumerate() {
			// set left on first non-pinned segment
			if i == line.get_number_of_pinned_segment() {
				left_start = left;
			}
			let (amount_drawn, segment_size) =
				segment.draw(left_start, window_width - start, line.get_selected(), &self.display);
			start += amount_drawn;
			if start >= window_width {
				break;
			}
			if amount_drawn > 0 {
				left_start = 0;
			}
			else {
				left_start -= segment_size;
			}
		}

		if start < window_width {
			let padding = " ".repeat(window_width - start);
			self.display.draw_str(padding.as_str());
		}
	}

	pub fn draw_title(&self, show_help: bool) {
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, true, false);
		let (window_width, _) = self.display.get_window_size();

		if window_width >= TITLE_LENGTH {
			self.display.draw_str(TITLE);
			// only draw help if there is room
			if window_width > TITLE_LENGTH + TITLE_HELP_INDICATOR_LENGTH {
				if (window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) > 0 {
					let padding = " ".repeat((window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) as usize);
					self.display.draw_str(padding.as_str());
				}
				if show_help {
					self.display
						.draw_str(format!("Help: {}", self.config.input_help).as_str());
				}
				else {
					let padding = " ".repeat(TITLE_HELP_INDICATOR_LENGTH as usize);
					self.display.draw_str(padding.as_str());
				}
			}
			else if (window_width - TITLE_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_LENGTH) as usize);
				self.display.draw_str(padding.as_str());
			}
		}
		else {
			self.display.draw_str(TITLE_SHORT);
			if (window_width - TITLE_SHORT_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_SHORT_LENGTH) as usize);
				self.display.draw_str(padding.as_str());
			}
		}
	}

	fn draw_vertical_spacer(&self, repeat: i32) {
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, false, false);
		for _x in 0..repeat {
			self.display
				.draw_str(format!("{}\n", self.config.theme.character_vertical_spacing).as_str());
		}
	}

	pub fn draw_prompt(&self, message: &str) {
		self.draw_title(false);
		self.display.set_style(false, false, false);
		self.display.draw_str(&format!("\n{} ", message));
	}

	pub fn draw_confirm(&self, message: &str) {
		self.draw_prompt(&format!(
			"{} ({}/{})? ",
			message, self.config.input_confirm_yes, self.config.input_confirm_no
		));
	}
}
