#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#[derive(Clone, Default)]
pub struct OutputTemplateBuilder
{
	template: String,
}

impl OutputTemplateBuilder
{
	pub fn new(template: String) -> Self
	{
		return Self
		{
			template
		};
	}
	
	pub fn clear(&mut self)
	{
		self.template.clear();
	}
	
	pub fn get(&self) -> String
	{
		return self.template.clone();
	}
	
	pub fn push(&mut self, variable: OutputTemplateVariable, join: Option<String>)
	{
		let s = self.formatVariable(variable);
		if let Some(j) = join
		{
			self.template.push_str(j.as_str());
		}
		self.template.push_str(s.as_str());
	}
	
	pub fn set(&mut self, template: String)
	{
		self.template = template.to_owned();
	}
	
	fn formatVariable(&self, variable: OutputTemplateVariable) -> String
	{
		let s = self.getVariableString(variable);
		return match variable
		{
			OutputTemplateVariable::IsLive
			| OutputTemplateVariable::WasLive
				=> format!("%({})b", s),
			
			OutputTemplateVariable::Timestamp
			| OutputTemplateVariable::ReleaseTimestamp
			| OutputTemplateVariable::ModifiedTimestamp
			| OutputTemplateVariable::ChannelFollowerCount
			| OutputTemplateVariable::Duration
			| OutputTemplateVariable::ViewCount
			| OutputTemplateVariable::ConcurrentViewCount
			| OutputTemplateVariable::LikeCount
			| OutputTemplateVariable::DislikeCount
			| OutputTemplateVariable::RepostCount
			| OutputTemplateVariable::AverageRating
			| OutputTemplateVariable::CommentCount
			| OutputTemplateVariable::AgeLimit
			| OutputTemplateVariable::StartTime
			| OutputTemplateVariable::EndTime
			| OutputTemplateVariable::Epoch
			| OutputTemplateVariable::Autonumber
			| OutputTemplateVariable::VideoAutonumber
			| OutputTemplateVariable::NEntries
			| OutputTemplateVariable::PlaylistCount
			| OutputTemplateVariable::PlaylistIndex
			| OutputTemplateVariable::PlaylistAutonumber
				=> format!("%({})n", s),
			
			_ => format!("%({})s", s),
		};
	}
	
	fn getVariableString(&self, variable: OutputTemplateVariable) -> String
	{
		return match variable
		{
			OutputTemplateVariable::Identifier					=> "id".to_string(),
			OutputTemplateVariable::Title						=> "title".to_string(),
			OutputTemplateVariable::FullTitle					=> "fulltitle".to_string(),
			OutputTemplateVariable::Extension					=> "ext".to_string(),
			OutputTemplateVariable::AlternateTitle				=> "alt_title".to_string(),
			OutputTemplateVariable::Description					=> "description".to_string(),
			OutputTemplateVariable::DisplayIdentifier			=> "display_id".to_string(),
			OutputTemplateVariable::Uploader					=> "uploader".to_string(),
			OutputTemplateVariable::License						=> "license".to_string(),
			OutputTemplateVariable::Creator						=> "creator".to_string(),
			OutputTemplateVariable::Timestamp					=> "timestamp".to_string(),
			OutputTemplateVariable::UploadDate					=> "upload_date".to_string(),
			OutputTemplateVariable::ReleaseTimestamp			=> "release_timestamp".to_string(),
			OutputTemplateVariable::ReleaseDate					=> "release_date".to_string(),
			OutputTemplateVariable::ModifiedTimestamp			=> "modified_timestamp".to_string(),
			OutputTemplateVariable::ModifiedDate				=> "modified_date".to_string(),
			OutputTemplateVariable::UploaderIdentifier			=> "uploader_id".to_string(),
			OutputTemplateVariable::Channel						=> "channel".to_string(),
			OutputTemplateVariable::ChannelIdentifier			=> "channel_id".to_string(),
			OutputTemplateVariable::ChannelFollowerCount		=> "channel_follower_count".to_string(),
			OutputTemplateVariable::Location					=> "location".to_string(),
			OutputTemplateVariable::Duration					=> "duration".to_string(),
			OutputTemplateVariable::DurationString				=> "duration_string".to_string(),
			OutputTemplateVariable::ViewCount					=> "view_count".to_string(),
			OutputTemplateVariable::ConcurrentViewCount			=> "concurrent_view_count".to_string(),
			OutputTemplateVariable::LikeCount					=> "like_count".to_string(),
			OutputTemplateVariable::DislikeCount				=> "dislike_count".to_string(),
			OutputTemplateVariable::RepostCount					=> "repost_count".to_string(),
			OutputTemplateVariable::AverageRating				=> "average_rating".to_string(),
			OutputTemplateVariable::CommentCount				=> "comment_count".to_string(),
			OutputTemplateVariable::AgeLimit					=> "age_limit".to_string(),
			OutputTemplateVariable::LiveStatus					=> "live_status".to_string(),
			OutputTemplateVariable::IsLive						=> "is_live".to_string(),
			OutputTemplateVariable::WasLive						=> "was_live".to_string(),
			OutputTemplateVariable::PlayableInEmbed				=> "playable_in_embed".to_string(),
			OutputTemplateVariable::Availability				=> "availability".to_string(),
			OutputTemplateVariable::StartTime					=> "start_time".to_string(),
			OutputTemplateVariable::EndTime						=> "end_time".to_string(),
			OutputTemplateVariable::Extractor					=> "extractor".to_string(),
			OutputTemplateVariable::ExtractorKey				=> "extractor_key".to_string(),
			OutputTemplateVariable::Epoch						=> "epoch".to_string(),
			OutputTemplateVariable::Autonumber					=> "autonumber".to_string(),
			OutputTemplateVariable::VideoAutonumber				=> "video_autonumber".to_string(),
			OutputTemplateVariable::NEntries					=> "n_entries".to_string(),
			OutputTemplateVariable::PlaylistIdentifier			=> "playlist_id".to_string(),
			OutputTemplateVariable::PlaylistTitle				=> "playlist_title".to_string(),
			OutputTemplateVariable::Playlist					=> "playlist".to_string(),
			OutputTemplateVariable::PlaylistCount				=> "playlist_count".to_string(),
			OutputTemplateVariable::PlaylistIndex				=> "playlist_index".to_string(),
			OutputTemplateVariable::PlaylistAutonumber			=> "playlist_autonumber".to_string(),
			OutputTemplateVariable::PlaylistUploader			=> "playlist_uploader".to_string(),
			OutputTemplateVariable::PlaylistUploaderIdentifier	=> "playlist_uploader_id".to_string(),
			OutputTemplateVariable::WebpageUrl					=> "webpage_url".to_string(),
			OutputTemplateVariable::WebpageUrlBasename			=> "webpage_url_basename".to_string(),
			OutputTemplateVariable::WebpageUrlDomain			=> "webpage_url_domain".to_string(),
			OutputTemplateVariable::OriginalUrl					=> "original_url".to_string(),
		};
	}
}

#[derive(Clone, Copy)]
pub enum OutputTemplateVariable
{
	Identifier,
	Title,
	FullTitle,
	Extension,
	AlternateTitle,
	Description,
	DisplayIdentifier,
	Uploader,
	License,
	Creator,
	Timestamp,
	UploadDate,
	ReleaseDate,
	ReleaseTimestamp,
	ModifiedTimestamp,
	ModifiedDate,
	UploaderIdentifier,
	Channel,
	ChannelIdentifier,
	ChannelFollowerCount,
	Location,
	Duration,
	DurationString,
	ViewCount,
	ConcurrentViewCount,
	LikeCount,
	DislikeCount,
	RepostCount,
	AverageRating,
	CommentCount,
	AgeLimit,
	LiveStatus,
	IsLive,
	WasLive,
	PlayableInEmbed,
	Availability,
	StartTime,
	EndTime,
	Extractor,
	ExtractorKey,
	Epoch,
	Autonumber,
	VideoAutonumber,
	NEntries,
	PlaylistIdentifier,
	PlaylistTitle,
	Playlist,
	PlaylistCount,
	PlaylistIndex,
	PlaylistAutonumber,
	PlaylistUploader,
	PlaylistUploaderIdentifier,
	WebpageUrl,
	WebpageUrlBasename,
	WebpageUrlDomain,
	OriginalUrl,
}
