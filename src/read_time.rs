use super::Options;

fn count_words(content: &str, options: &Options) -> f32
{
	let mut count: f32 = 0.0;
	for word in content.split(" ") {
		count += (word.len() as f32)/options.get_word_length() as f32;
	}
	count.round()
}

pub fn calculate_read_time(content: &str, options: &Options) -> ReadTime
{
	let word_count = count_words(content, options);
	let seconds = (word_count / options.get_wpm() as f32) * 60.0;

	let word_count = word_count as u64;
	let seconds = seconds.round() as u64;

	ReadTime::new(word_count, seconds)
}

/// Response returned by the crate after calculating the avg. time taken to read a text.
pub struct ReadTime {
	word_count: u64,
	seconds: u64
}

impl ReadTime {
	fn new(word_count: u64, seconds: u64) -> ReadTime {
		ReadTime {
			word_count,
			seconds
		}
	}

	/// Returns the total words counted by the crate for the content passed.
	pub fn word_count(&self) -> u64 {
		self.word_count
	}

	/// Returns the total seconds taken to read the content.
	pub fn seconds(&self) -> u64 {
		self.seconds
	}
}