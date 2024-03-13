use std::time::UNIX_EPOCH as UNIX_EPOCH;
use std::time::SystemTime as SystemTime;

#[derive (Clone, Debug)]
pub struct Flashcard {
	pub shown_side: String,
	pub hidden_side: String,
	pub timestamp_for_next_repetition: u64,
	pub repetition_number: u16,
	pub easiness_factor: f32,
	pub interval: u16
}

impl Flashcard {
	pub fn update_stats(&self, user_grade: u8) -> Flashcard {
    		let shown_side = self.shown_side.clone();
    		let hidden_side = self.hidden_side.clone();
		let mut repitition_number = self.repetition_number;
    		let mut easiness_factor = self.easiness_factor;
    		let mut interval = self.interval;

    		if user_grade >= 3 { // Correctly Recalled
			if repitition_number == 0 {
				interval = 1;
			} else if repitition_number == 1 {
				interval = 6;
			} else {
				interval = (interval as f32 * easiness_factor).round() as u16;
			}
			repitition_number += 1;
    		} else {
			repitition_number = 0;
			interval = 1
    		}

    		easiness_factor += 0.1 - (5. - user_grade as f32) * (0.08 + (5. - user_grade as f32) * 0.02);
    		if easiness_factor < 1.3 {
			easiness_factor = 1.3;
    		}

		let timestamp_for_next_repetition: u64;
		let current_time: u64;
		if user_grade >= 4 {
			current_time = (SystemTime::now().duration_since(UNIX_EPOCH)).expect("REASON").as_secs();
			timestamp_for_next_repetition = current_time + (86400 * interval as u64) - 28800;
		} else {
			timestamp_for_next_repetition = 0;
		};

		Flashcard {
			shown_side,
			hidden_side,
			timestamp_for_next_repetition,
			repetition_number: repitition_number,
			easiness_factor,
			interval
		}
	}


	pub fn is_up_for_repetition(&self) -> bool {
		let current_time: u64 = (SystemTime::now().duration_since(UNIX_EPOCH)).expect("REASON").as_secs();
		let timestamp_for_next_repetition = self.timestamp_for_next_repetition;
		timestamp_for_next_repetition < current_time
	}
}

pub fn create_flashcard(shown_side: &str, hidden_side: &str) -> Flashcard {
	Flashcard {
		shown_side: shown_side.to_string(),
		hidden_side: hidden_side.to_string(),
		timestamp_for_next_repetition: 0,
		repetition_number: 0,
		easiness_factor: 2.5,
		interval: 0
	}
}
