use std::collections::VecDeque;
use crate::flashcards;
use crate::readwrite;

#[derive(Debug)]
pub struct FlashcardsToLearn {
	queue: VecDeque<flashcards::Flashcard>,
}

impl FlashcardsToLearn {
	pub fn add_flashcard(&mut self, flashcard: flashcards::Flashcard) {
		self.queue.push_back(flashcard); 
	}

	pub fn get_flashcard(&mut self) -> flashcards::Flashcard {
		self.queue.pop_front().expect("Flashcard In Queue")
	}

	pub fn get_flashcards_left(&self) -> usize {
		self.queue.len()
	}
}

fn create_flashcards_to_learn() -> FlashcardsToLearn {
	FlashcardsToLearn {
		queue: VecDeque::new()
	}
}

pub fn load_flashcards(path: &str) -> (FlashcardsToLearn, Vec<flashcards::Flashcard>) {
	let flashcards: Vec<flashcards::Flashcard> = readwrite::read_flashcards_from_file(path);
	let mut flashcards_to_learn = create_flashcards_to_learn();
	let mut unused_flashcards: Vec<flashcards::Flashcard> = Vec::new();
	for flashcard in flashcards.iter() {
		if flashcard.is_up_for_repetition() {
			flashcards_to_learn.add_flashcard(flashcard.clone());
		} else {
			unused_flashcards.push(flashcard.clone());
		}
	}
	(flashcards_to_learn,unused_flashcards)
}
