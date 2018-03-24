/* 
 * This pub struct is built entirely from the TOML strings in the /lang files, and will need to be updated EVERY time that new variables are added.
 * Fortunately there is a fairly easy set of steps that can be done to achieve this:
 * Remove quoted text strings with:[ = \"(.*?)\"+ ], and replace them with: [: String,]
 */

#[derive(Serialize, Deserialize)]
// multipurpose
pub struct Lang_multipurpose {
	instance_title: String,
	go_back: String,
	submit: String,
	comment_placeholder: String,
}

// aside_settings
pub struct Lang_aside_settings {
	settings: String,
	edit_profile: String,
	preferences: String,
	mute_words: String,
	mute_users: String,
	notifications: String,
	security: String,
	data_import: String,
	data_export: String,
	auth_apps: String,
	auth_follow: String,
	invites: String,
}

// aside_shortcuts
pub struct Lang_aside_shortcuts {
	news: String,
	messages: String,
	shortcuts: String,
	calendar: String,
	groups: String,
	lists: String,
	photos: String,
	favorites: String,
	weather: String,
	create: String,
	new_event: String,
	new_group: String,
}

// profile_edit
pub struct Lang_profile_edit {
	profile_title: String,
	profile_sub_title: String,
	profile_display_name: String,
	profile_char_remaining: String,
	profile_bio: String,
	profile_avatar_upload: String,
	profile_avatar_upload_btn_label: String,
	profile_avatar_file_name: String,
	profile_avatar_sub_text: String,
	profile_header_upload: String,
	profile_avatar_header_label: String,
	profile_header_file_name: String,
	profile_header_sub_text: String,
}

// messaging
pub struct Lang_aside_messaging {
	enter_post: String,
	post: String,
	reply: String,
}

// footer
pub struct Lang_aside_footer {
	terms: String,
	copyright: String,
	github: String,
	donate: String,
}
