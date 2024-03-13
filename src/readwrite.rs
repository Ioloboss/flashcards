use crate::flashcards::Flashcard;
use crate::flashcards::create_flashcard;
use std::fs;
use  csv;

pub fn flashcard_to_binary(flashcard: Flashcard) -> Vec<u8> {
	let shown_side_length: u8 = flashcard.shown_side.len().try_into().unwrap();
	let hidden_side_length: u8 = flashcard.hidden_side.len().try_into().unwrap();
	let mut binary_data = Vec::new();

	binary_data.push(shown_side_length);
	binary_data.extend(&(flashcard.shown_side).as_bytes().to_vec());
	binary_data.push(hidden_side_length);
	binary_data.extend(&(flashcard.hidden_side).as_bytes().to_vec());

	binary_data.extend(flashcard.timestamp_for_next_repetition.to_be_bytes().to_vec());
	binary_data.extend(flashcard.repetition_number.to_be_bytes().to_vec());
	binary_data.extend(flashcard.easiness_factor.to_be_bytes().to_vec());
	binary_data.extend(flashcard.interval.to_be_bytes().to_vec());
	
	binary_data
}

pub fn binary_to_flashcard(binary: Vec<u8>) -> Flashcard {
	let shown_side_length: u8 = binary[0];
	let shown_side_start: u16 = 1;
	let shown_side_end: u16 = shown_side_start + shown_side_length as u16;
	let shown_side_as_vec: Vec<u8> = (binary[shown_side_start as usize..shown_side_end as usize]).to_vec();

	let hidden_side_length: u8 = binary[shown_side_end as usize];
	let hidden_side_start: u16 = shown_side_end + 1;
	let hidden_side_end: u16 = hidden_side_start + hidden_side_length as u16;
	let hidden_side_as_vec: Vec<u8> = (binary[hidden_side_start as usize..hidden_side_end as usize]).to_vec();

	let timestamp_start: u16 = hidden_side_end;
	let timestamp_end: u16 = timestamp_start + 8;
	let timestamp_as_array_of_bytes: [u8;8] = (&binary[timestamp_start as usize..timestamp_end as usize]).try_into().unwrap();

	let repetition_number_start: u16 = timestamp_end;
	let repetition_number_end: u16 = repetition_number_start + 2;
	let repetition_number_as_array_of_bytes: [u8;2] = (&binary[repetition_number_start as usize..repetition_number_end as usize]).try_into().unwrap();

	let easiness_factor_start: u16 = repetition_number_end;
	let easiness_factor_end: u16 = easiness_factor_start + 4;
	let easiness_factor_as_array_of_bytes: [u8;4] = (&binary[easiness_factor_start as usize..easiness_factor_end as usize]).try_into().unwrap();

	let interval_start: u16 = easiness_factor_end;
	let interval_end: u16 = interval_start + 2;
	let interval_as_array_of_bytes: [u8;2] = (&binary[interval_start as usize..interval_end as usize]).try_into().unwrap();

	let shown_side: String = String::from_utf8(shown_side_as_vec).expect("valid utf8");
	let hidden_side: String = String::from_utf8(hidden_side_as_vec).expect("valid utf8");

	let timestamp_for_next_repetition = u64::from_be_bytes(timestamp_as_array_of_bytes);
	let repetition_number = u16::from_be_bytes(repetition_number_as_array_of_bytes);
	let easiness_factor = f32::from_be_bytes(easiness_factor_as_array_of_bytes);
	let interval = u16::from_be_bytes(interval_as_array_of_bytes);

	Flashcard {
		shown_side,
		hidden_side,
		timestamp_for_next_repetition,
		repetition_number,
		easiness_factor,
		interval
	}
}

pub fn flashcards_to_full_binary(flashcards: Vec<Flashcard>) -> Vec<u8> {
	let mut full_binary_data: Vec<u8> = Vec::new();
	for flashcard in flashcards.into_iter() {
		let flashcard_as_binary = flashcard_to_binary(flashcard);
		let flashcard_as_binary_length: [u8;2] = (flashcard_as_binary.len() as u16).to_be_bytes();
		full_binary_data.extend(flashcard_as_binary_length.to_vec());
		full_binary_data.extend(flashcard_as_binary);
	}
	full_binary_data
}

pub fn full_binary_to_flashcards(full_binary_data: Vec<u8>) -> Vec<Flashcard> {
	let mut flashcards: Vec<Flashcard> = Vec::new();
    	let mut position: u32 = 0;
	let mut reached_end: bool = false;
	let full_binary_data_length: u32 = full_binary_data.len() as u32;

	while !reached_end {
		let flashcard_length_as_bytes: [u8;2] = (&full_binary_data[position as usize..=(position+1) as usize]).try_into().unwrap();
		let flashcard_length: u16 = u16::from_be_bytes(flashcard_length_as_bytes);
		let flashcard_start: u32 = position + 2;
		let flashcard_end: u32 = flashcard_start + flashcard_length as u32;
		let flashcard_as_binary: Vec<u8> = full_binary_data[flashcard_start as usize..flashcard_end as usize].to_vec();
		let flashcard: Flashcard = binary_to_flashcard(flashcard_as_binary);
		flashcards.push(flashcard);
		position += flashcard_length as u32 + 2;
		if position >= full_binary_data_length {
			reached_end = true;
		}
	}
	
	flashcards
}

pub fn write_flashcards_to_file(flashcards: Vec<Flashcard>, path: &str) {
	let flashcards_as_binary: Vec<u8> = flashcards_to_full_binary(flashcards);
	fs::write(path, flashcards_as_binary).expect("Unable to write data");
}

pub fn read_flashcards_from_file(path: &str) -> Vec<Flashcard> {
	let flashcards_as_binary: Vec<u8> = fs::read(path).expect("Unable to read data");
	full_binary_to_flashcards(flashcards_as_binary)
}

pub fn read_from_csv_file(path: &str) -> Vec<Flashcard> {
	let csv_data: String = fs::read_to_string(path).expect("Unable to read csv");
	let mut flashcards: Vec<Flashcard> = Vec::new();
	let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
	for result in reader.records() {
    		let record = result.expect("csv reading");
		let flashcard = create_flashcard(&record[0], &record[1]);
		flashcards.push(flashcard);
	}
	flashcards
}
