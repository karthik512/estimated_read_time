mod read_time;
mod read_options;

pub use read_options::Options;
pub use read_time::ReadTime;

/// Calculates the time taken to read the content passed.
///
/// # Arguments
///
/// * `content`	- The content to read.
///
/// * `options`	- The options object provided by `estimated_read_time::Options` which contains some configuration on how to calculate the time taken to read the content.
///
/// # Returns
///
/// `estimated_read_time::ReadTime`	- The ReadTime object which contains the time taken to read the content.
///
/// # Examples
///
/// ```
/// use estimated_read_time::Options
/// use estimated_read_time::ReadTime
///
/// ///Example 1
/// let read_norm_doc = Options::new()
///				.word_length(5)
///				.wpm(300)
///				.build()
///				.unwrap_or_default();
///	let time_taken: ReadTime = estimated_read_time::text(&content, &read_norm_doc);
/// 
/// let word_count: u64 = time_taken.word_count();
/// let seconds: u64 = time_taken.seconds();
///
///	///Example 2
///	let read_tech_doc = Options::new()
///				.word_length(5)
///				.technical_document(true)
///				.technical_difficulty(2)
///				.build()
///				.unwrap_or_default();
/// let time_taken: ReadTime = estimated_read_time::text(&content, &read_tech_doc);
///
/// let word_count: u64 = time_taken.word_count();
/// let seconds: u64 = time_taken.seconds();
/// ```
pub fn text(content: &str, options: &Options) -> ReadTime
{
	let read_time = read_time::calculate_read_time(content, options);
	read_time
}