pub fn help(args: Vec<String>) {
	println!("Flashcards -- A Spaced Repetition Flashcard Prgram");
	println!("Layout \'{} flc_file options ...\'", &args[0]);
	println!("Options:");
	println!("    help - brings up this message");
	println!("    import - import flashcards from csv");
	println!("    `no option` - loads the program with specified flashcards file");
}
