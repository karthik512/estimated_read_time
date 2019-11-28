use std::convert::TryInto;
use std::default::Default;

/// Some Options to be set which decides the calculation of avg. time taken to read a text.
pub struct Options
{
	word_length: u32,
	wpm: u32,
	is_technical_document: bool,
	technical_difficulty: u8,
	total_words: u64,
	total_seconds: u64
}

impl Options
{
	///Creates a new instance of Options with default values set. For Default Values, refer the documentation of each method below.
	pub fn new() -> Options
	{
		Options {
			word_length: 4,
			wpm: 265,
			is_technical_document: false,
			technical_difficulty: 3,
			total_words: 0,
			total_seconds: 0
		}
	}

	///Avg. length of one word which will be used to count the total no. of word in the content passed.<br>
	///
	/// # Arguments
	///
	/// * `word_length`	- The word length to be considered to count the total no. of words. `[Default Value - 4]`
	///
	pub fn word_length(mut self, word_length: u32) -> Self
	{
		self.word_length = word_length;
		self
	}

	///Returns the avg. word length used to compute the total no. of words in a text.
	pub fn get_word_length(&self) -> u32
	{
		self.word_length
	}

	/// The Avg. WPM (Words per minute) a person can read or taken to read a text
	///
	/// # Arguments
	///
	/// * `wpm`	- The Avg. WPM to be considered for the text. `[Default Value - 265]`
	///
	pub fn wpm(mut self, wpm: u32) -> Self
	{
		self.wpm = wpm;
		self
	}

	/// Returns the avg. WPM
	pub fn get_wpm(&self) -> u32
	{
		self.wpm
	}

	/// Sets whether the text is a technical document or not. Setting this to true assumes that reading a technical document takes more time than a simple document.
	///
	/// # Arguments
	///
	/// * `is_technical_document`	- true/false. `[Default Value - false]`
	///
	pub fn technical_document(mut self, is_technical_document: bool) -> Self
	{
		self.is_technical_document = is_technical_document;
		self
	}

	/// Returns whether it is technical document.
	pub fn is_technical_document(&self) -> bool
	{
		self.is_technical_document
	}

	/// Sets the technical difficulty of the document. More the difficuly, more time to read.
	///
	/// # Arguments
	///
	/// * `technical_difficulty`	- Technical Difficulty value in the range [1, 5]. `[Default Value - 3]`
	///
	pub fn technical_difficulty(mut self, technical_difficulty: u8) -> Self
	{
		self.technical_difficulty = technical_difficulty;		
		self
	}

	/// Calculates the final WPM based on previous WPM and Technical Difficulty
	fn calculate_wpm(&self) -> u32
	{
		let mut new_wpm = self.wpm;
		if self.total_words > 0 && self.total_seconds > 0 {
			new_wpm = (((self.total_words * 60) / self.total_seconds)).try_into().unwrap();
		}
		if self.is_technical_document {
			let t_wpm = self.wpm - (65 + (30 * self.technical_difficulty as u32));
			new_wpm = if t_wpm <= 0 { 50 } else { t_wpm };
		}
		if self.total_words <= 0 || self.total_seconds <= 0 {
			if !self.is_technical_document {
				new_wpm = 265;
			}
		}
		new_wpm
	}

	/// Returns the Technical Difficulty value.
	pub fn get_technical_difficulty(&self) -> u8
	{
		self.technical_difficulty
	}

	/// Total Words read in Total Seconds previously. If this is set, it will be used to calculate WPM.
	/// This Calculated WPM takes higher precedence than the default WPM and WPM set using wpm().
	///
	/// This can be useful when maintaining state for individual users.
	///
	/// # Arguments
	///
	/// * `total_words`		- Total Words read. `[Default Value - 0]`
	/// * `total_seconds`	- Total seconds taken to read the total_words. `[Default Value - 0]`
	///
	pub fn previous_read_time(mut self, total_words: u64, total_seconds: u64) -> Self
	{
		self.total_words = total_words;
		self.total_seconds = total_seconds;
		self
	}

	pub fn build(self) -> Result<Options, String> {
		if self.technical_difficulty < 1 || self.technical_difficulty > 5 {
			return Err("Technical Difficulty must be in the range [1, 5]".to_string());
		}
		Ok(Options {
			word_length: self.word_length,
			wpm: self.calculate_wpm(),
			is_technical_document: self.is_technical_document,
			technical_difficulty: self.technical_difficulty,
			total_words: self.total_words,
			total_seconds: self.total_seconds						
		})	
	}
}

///Returns the Options object with the default values set.
impl Default for Options
{
	fn default() -> Options {
		Options::new()
	}
}