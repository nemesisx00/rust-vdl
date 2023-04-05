#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#[derive(Clone, Default)]
pub struct TemplateBuilder
{
	template: String,
}

impl TemplateBuilder
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
	
	pub fn push(&mut self, variable: TemplateVariable, join: Option<String>)
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
	
	fn formatVariable(&self, variable: TemplateVariable) -> String
	{
		let s = self.getVariableString(variable);
		return match variable
		{
			TemplateVariable::IsLive
			| TemplateVariable::WasLive
				=> format!("%({})b", s),
			
			TemplateVariable::Timestamp
			| TemplateVariable::ReleaseTimestamp
			| TemplateVariable::ModifiedTimestamp
			| TemplateVariable::ChannelFollowerCount
			| TemplateVariable::Duration
			| TemplateVariable::ViewCount
			| TemplateVariable::ConcurrentViewCount
			| TemplateVariable::LikeCount
			| TemplateVariable::DislikeCount
			| TemplateVariable::RepostCount
			| TemplateVariable::AverageRating
			| TemplateVariable::CommentCount
			| TemplateVariable::AgeLimit
			| TemplateVariable::StartTime
			| TemplateVariable::EndTime
			| TemplateVariable::Epoch
			| TemplateVariable::Autonumber
			| TemplateVariable::VideoAutonumber
			| TemplateVariable::NEntries
			| TemplateVariable::PlaylistCount
			| TemplateVariable::PlaylistIndex
			| TemplateVariable::PlaylistAutonumber
				=> format!("%({})n", s),
			
			_ => format!("%({})s", s),
		};
	}
	
	fn getVariableString(&self, variable: TemplateVariable) -> String
	{
		return match variable
		{
			TemplateVariable::Identifier					=> "id".to_string(),
			TemplateVariable::Title							=> "title".to_string(),
			TemplateVariable::FullTitle						=> "fulltitle".to_string(),
			TemplateVariable::Extension						=> "ext".to_string(),
			TemplateVariable::AlternateTitle				=> "alt_title".to_string(),
			TemplateVariable::Description					=> "description".to_string(),
			TemplateVariable::DisplayIdentifier				=> "display_id".to_string(),
			TemplateVariable::Uploader						=> "uploader".to_string(),
			TemplateVariable::License						=> "license".to_string(),
			TemplateVariable::Creator						=> "creator".to_string(),
			TemplateVariable::Timestamp						=> "timestamp".to_string(),
			TemplateVariable::UploadDate					=> "upload_date".to_string(),
			TemplateVariable::ReleaseTimestamp				=> "release_timestamp".to_string(),
			TemplateVariable::ReleaseDate					=> "release_date".to_string(),
			TemplateVariable::ModifiedTimestamp				=> "modified_timestamp".to_string(),
			TemplateVariable::ModifiedDate					=> "modified_date".to_string(),
			TemplateVariable::UploaderIdentifier			=> "uploader_id".to_string(),
			TemplateVariable::Channel						=> "channel".to_string(),
			TemplateVariable::ChannelIdentifier				=> "channel_id".to_string(),
			TemplateVariable::ChannelFollowerCount			=> "channel_follower_count".to_string(),
			TemplateVariable::Location						=> "location".to_string(),
			TemplateVariable::Duration						=> "duration".to_string(),
			TemplateVariable::DurationString				=> "duration_string".to_string(),
			TemplateVariable::ViewCount						=> "view_count".to_string(),
			TemplateVariable::ConcurrentViewCount			=> "concurrent_view_count".to_string(),
			TemplateVariable::LikeCount						=> "like_count".to_string(),
			TemplateVariable::DislikeCount					=> "dislike_count".to_string(),
			TemplateVariable::RepostCount					=> "repost_count".to_string(),
			TemplateVariable::AverageRating					=> "average_rating".to_string(),
			TemplateVariable::CommentCount					=> "comment_count".to_string(),
			TemplateVariable::AgeLimit						=> "age_limit".to_string(),
			TemplateVariable::LiveStatus					=> "live_status".to_string(),
			TemplateVariable::IsLive						=> "is_live".to_string(),
			TemplateVariable::WasLive						=> "was_live".to_string(),
			TemplateVariable::PlayableInEmbed				=> "playable_in_embed".to_string(),
			TemplateVariable::Availability					=> "availability".to_string(),
			TemplateVariable::StartTime						=> "start_time".to_string(),
			TemplateVariable::EndTime						=> "end_time".to_string(),
			TemplateVariable::Extractor						=> "extractor".to_string(),
			TemplateVariable::ExtractorKey					=> "extractor_key".to_string(),
			TemplateVariable::Epoch							=> "epoch".to_string(),
			TemplateVariable::Autonumber					=> "autonumber".to_string(),
			TemplateVariable::VideoAutonumber				=> "video_autonumber".to_string(),
			TemplateVariable::NEntries						=> "n_entries".to_string(),
			TemplateVariable::PlaylistIdentifier			=> "playlist_id".to_string(),
			TemplateVariable::PlaylistTitle					=> "playlist_title".to_string(),
			TemplateVariable::Playlist						=> "playlist".to_string(),
			TemplateVariable::PlaylistCount					=> "playlist_count".to_string(),
			TemplateVariable::PlaylistIndex					=> "playlist_index".to_string(),
			TemplateVariable::PlaylistAutonumber			=> "playlist_autonumber".to_string(),
			TemplateVariable::PlaylistUploader				=> "playlist_uploader".to_string(),
			TemplateVariable::PlaylistUploaderIdentifier	=> "playlist_uploader_id".to_string(),
			TemplateVariable::WebpageUrl					=> "webpage_url".to_string(),
			TemplateVariable::WebpageUrlBasename			=> "webpage_url_basename".to_string(),
			TemplateVariable::WebpageUrlDomain				=> "webpage_url_domain".to_string(),
			TemplateVariable::OriginalUrl					=> "original_url".to_string(),
		};
	}
}

#[derive(Clone, Copy)]
pub enum TemplateVariable
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
