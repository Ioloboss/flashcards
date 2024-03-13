mod flashcards;
mod readwrite;
mod help;
mod running_collections;
mod tui;

use std::env;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
	prelude::*,
	symbols::border,
	widgets::{block::*, *},
};

#[derive(PartialEq, Eq)]
enum Stage {
	Start,
	DisplayingShownSide,
	DisplayingHiddenSide,
	AllCardsDone
}

struct Application {
	queue: running_collections::FlashcardsToLearn,
	current_flashcard: flashcards::Flashcard,
	stage: Stage,
	exit: bool,
	unused_flashcards: Vec<flashcards::Flashcard>,
}

impl Application {
	pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
		if self.queue.get_flashcards_left() == 0 {
			self.stage = Stage::AllCardsDone;
		}
		while !self.exit {
    			if self.stage == Stage::Start {
				self.current_flashcard = self.queue.get_flashcard();
				self.stage = Stage::DisplayingShownSide;
    			}
			terminal.draw(|frame| self.render_frame(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	fn render_frame(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.size());
	}

	fn handle_events(&mut self) -> io::Result<()> {
		match event::read()? {
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			},
			_ => {},
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
    		match self.stage {
        		Stage::AllCardsDone => {
        			match key_event.code {
					KeyCode::Char('q') => self.exit(),
					_ => {}
        			}
        		},
        		Stage::DisplayingShownSide => {
				match key_event.code {
					KeyCode::Char('q') => self.exit(),
					KeyCode::Char(' ')  => self.show_hidden(),
					_ => {}
				}
        		},
        		Stage::DisplayingHiddenSide => {
				match key_event.code {
					KeyCode::Char('q') => self.exit(),
					KeyCode::Char('0') => self.update_flashcard(0),
					KeyCode::Char('1') => self.update_flashcard(1),
					KeyCode::Char('2') => self.update_flashcard(2),
					KeyCode::Char('3') => self.update_flashcard(3),
					KeyCode::Char('4') => self.update_flashcard(4),
					KeyCode::Char('5') => self.update_flashcard(5),
					_ => {}
				}
        		},
        		Stage::Start => {},
		}
	}

	fn exit(&mut self) {
		self.exit = true;
		for i in 0..self.queue.get_flashcards_left() {
			self.unused_flashcards.push(self.queue.get_flashcard());
		}
		if self.stage != Stage::AllCardsDone {
			self.unused_flashcards.push(self.current_flashcard.clone());
		}
	}

	fn show_hidden(&mut self) {
		self.stage = Stage::DisplayingHiddenSide;
	}

	fn update_flashcard(&mut self, user_grade: u8) {
		let flashcards_remaining = self.queue.get_flashcards_left();
		if user_grade >= 4 {
			self.unused_flashcards.push(self.current_flashcard.clone().update_stats(user_grade));
		} else {
			self.queue.add_flashcard(self.current_flashcard.clone().update_stats(user_grade));
		}
		if flashcards_remaining > 0 {
			self.current_flashcard = self.queue.get_flashcard();
			self.stage = Stage::DisplayingShownSide;
		} else {
			self.stage = Stage::AllCardsDone;
		}
	}
}

impl Widget for &Application {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let title = Title::from(" Spaced Repetition Flashcards ".bold());
		let instructions: Title;
		match self.stage {
			Stage::DisplayingShownSide => {
				instructions = Title::from(Line::from(vec![
					" Show Back ".into(),
					" <Space> ".blue().bold(),
					" Quit ".into(),
					" <Q> ".blue().bold(),
    				]));
			},
			Stage::DisplayingHiddenSide => {
    				instructions = Title::from(Line::from(vec![
					" Difficulty ".into(),
					" <0..5> ".blue().bold(),
					" Quit ".into(),
					" <Q> ".blue().bold(),
        			]));
			},
			Stage::AllCardsDone => {
				instructions = Title::from(Line::from(vec![
					" Quit ".into(),
					" <Q> ".blue().bold(),
        			]));
			},
			Stage::Start => {
				instructions = Title::from(Line::from(vec![
					" Wait ".into(),
    				]));
			},
		};

		let block = Block::default()
    			.title(title.alignment(Alignment::Center))
    			.title(
				instructions
    					.alignment(Alignment::Center)
    					.position(Position::Bottom),
    			)
    			.borders(Borders::ALL)
    			.border_set(border::THICK);

		let centre_text: Text;
		match self.stage {
			Stage::AllCardsDone => centre_text = Text::from("All cards for this session are done."),
			Stage::DisplayingShownSide => centre_text = Text::from(self.current_flashcard.shown_side.clone()),
			Stage::DisplayingHiddenSide => centre_text = Text::from(self.current_flashcard.hidden_side.clone()),
			Stage::Start => centre_text = Text::from("Starting..."),
		};

		Paragraph::new(centre_text)
    			.centered()
    			.block(block)
    			.render(area, buf);
	}
}

fn import_from_csv(csv_path: &str, flc_path: &str) {
	let flashcards: Vec <flashcards::Flashcard> = readwrite::read_from_csv_file(csv_path);
	let number_of_flashcards = flashcards.len();
	readwrite::write_flashcards_to_file(flashcards, flc_path);
	println!("Imported {} flashcards from {} to {}", number_of_flashcards, csv_path, flc_path);
}

fn main() -> io::Result<()> {

	let args: Vec<String> = env::args().collect();
	let argc: usize = args.len();
	let flc_path: &str = &args[1];
	if argc == 1 {
		println!("needs path to flashcards file");
	} else if flc_path == "help" {
    		help::help(args);
	} else if argc > 2 {
    		let option: &str = &args[2];
		match option {
			"import" => import_from_csv(&args[3], flc_path),
			"help" => help::help(args),
			_ => println!("Incorrect Arguments")
			
		}
	} else {
		//let flashcards2: Vec<flashcards::Flashcard> = readwrite::read_flashcards_from_file(flc_path);
		//println!("from binary {:?}", flashcards2);

		let mut terminal = tui::init()?;
		let (used_flashcards, unused_flashcards) = running_collections::load_flashcards(flc_path);
		let mut application = Application {
			queue: used_flashcards,
			current_flashcard: flashcards::create_flashcard(" SOMETHING IS WRONG ", " SOMETHING IS WRONG "),
			stage: Stage::Start,
			exit: false,
			unused_flashcards,
		};
    		let app_result = application.run(&mut terminal);
    		readwrite::write_flashcards_to_file(application.unused_flashcards, flc_path);
    		tui::restore()?;
    		return app_result;
	}
	Ok(())
}
