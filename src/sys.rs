// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

#![allow(non_camel_case_types, non_upper_case_globals)]

use libc::{c_void, c_int, c_uint, c_char, c_float, uintptr_t, FILE};
use libloading_mini::Library;

pub type c_bool = u8;

pub type libvlc_event_type_t = c_int;

// From libvlc_structures.h
pub enum libvlc_instance_t {}
pub enum libvlc_log_iterator_t {}

pub type libvlc_time_t = i64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_log_message_t {
    pub i_severity: c_int,
    pub psz_type: *const c_char,
    pub psz_name: *const c_char,
    pub psz_header: *const c_char,
    pub psz_message: *const c_char,
}

// From libvlc.h
pub enum libvlc_event_manager_t {}
pub enum libvlc_log_t {}
pub enum vlc_log_t {}

pub type libvlc_callback_t = unsafe extern "C" fn(*const libvlc_event_t, *mut c_void);
pub type va_list = *mut c_void;
pub type libvlc_log_cb = unsafe extern "C" fn(*mut c_void, c_int, *const libvlc_log_t, *const c_char, va_list);

pub use crate::enums::LogLevel as libvlc_log_level;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_module_description_t {
    pub psz_name: *const c_char,
    pub psz_shortname: *const c_char,
    pub psz_longname: *const c_char,
    pub psz_help: *const c_char,
    pub p_next: *mut libvlc_module_description_t,
}

pub unsafe fn libvlc_delay(pts: i64) -> i64 {
    pts - (get_vlc_dll().libvlc_clock)()
}

// From libvlc_media.h
pub enum libvlc_media_t {}

pub use crate::enums::Meta as libvlc_meta_t;
pub use crate::enums::State as libvlc_state_t;

pub const libvlc_media_option_trusted: u32 = 0x2;
pub const libvlc_media_option_unique: u32 = 0x100;

pub use crate::enums::TrackType as libvlc_track_type_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_stats_t {
    /* Input */
    pub i_read_bytes: c_int, pub f_input_bitrate: c_float, /* Demux */
    pub i_demux_read_bytes: c_int, pub f_demux_bitrate: c_float, pub i_demux_corrupted: c_int, pub i_demux_discontinuity: c_int, /* Decoders */
    pub i_decoded_video: c_int, pub i_decoded_audio: c_int, /* Video Output */
    pub i_displayed_pictures: c_int, pub i_lost_pictures: c_int, /* Audio output */
    pub i_played_abuffers: c_int, pub i_lost_abuffers: c_int, /* Stream output */
    pub i_sent_packets: c_int, pub i_sent_bytes: c_int, pub f_send_bitrate: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_track_info_t {
    /* Codec fourcc */
    pub i_codec: u32,
    pub i_id: c_int,
    pub i_type: libvlc_track_type_t, /* Codec specific */
    pub i_profile: c_int,
    pub i_level: c_int,
    pub u: libvlc_media_track_info_t_types::u,
}

pub mod libvlc_media_track_info_t_types {
    use libc::c_uint;
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub audio: audio,
        pub video: video,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct audio {
        pub i_channels: c_uint,
        pub i_rate: c_uint,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct video {
        pub i_height: c_uint,
        pub i_width: c_uint,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_track_t {
    pub i_channels: c_uint,
    pub i_rate: c_uint,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct libvlc_video_track_t {
    pub i_height: c_uint,
    pub i_width: c_uint,
    pub i_sar_num: c_uint,
    pub i_sar_den: c_uint,
    pub i_frame_rate_num: c_uint,
    pub i_frame_rate_den: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_subtitle_track_t {
    pub psz_encoding: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_track_t {
    pub i_codec: u32,
    pub i_original_fourcc: u32,
    pub i_id: c_int,
    pub i_type: libvlc_track_type_t,
    pub i_profile: c_int,
    pub i_level: c_int,
    pub u: libvlc_media_track_t_types::u,
    pub i_bitrate: c_uint,
    pub psz_language: *mut c_char,
    pub psz_description: *mut c_char,
}

pub mod libvlc_media_track_t_types {
    use super::*;
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub audio: *mut libvlc_audio_track_t,
        pub video: *mut libvlc_video_track_t,
        pub subtitle: *mut libvlc_subtitle_track_t,
    }
}

impl libvlc_media_track_t {
    pub unsafe fn audio(&self) -> *mut libvlc_audio_track_t {
        self.u.audio
    }
    pub unsafe fn video(&self) -> *mut libvlc_video_track_t {
        self.u.video
    }
    pub unsafe fn subtitle(&self) -> *mut libvlc_subtitle_track_t {
        self.u.subtitle
    }
}

// From libvlc_media_player.h

pub enum libvlc_media_player_t {}
pub enum libvlc_equalizer_t {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct libvlc_track_description_t {
    pub i_id: c_int,
    pub psz_name: *mut c_char,
    pub p_next: *mut libvlc_track_description_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_output_t {
    pub psz_name: *mut c_char,
    pub psz_description: *mut c_char,
    pub p_next: *mut libvlc_audio_output_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_output_device_t {
    pub p_next: *mut libvlc_audio_output_device_t,
    pub psz_device: *mut c_char,
    pub psz_description: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct libvlc_rectangle_t {
    pub top: c_int,
    pub left: c_int,
    pub bottom: c_int,
    pub right: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_video_marquee_option_t {
    libvlc_marquee_Enable = 0,
    libvlc_marquee_Text,
    libvlc_marquee_Color,
    libvlc_marquee_Opacity,
    libvlc_marquee_Position,
    libvlc_marquee_Refresh,
    libvlc_marquee_Size,
    libvlc_marquee_Timeout,
    libvlc_marquee_X,
    libvlc_marquee_Y,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_navigate_mode_t {
    libvlc_navigate_activate = 0,
    libvlc_navigate_up,
    libvlc_navigate_down,
    libvlc_navigate_left,
    libvlc_navigate_right,
}

pub use crate::enums::Position as libvlc_position_t;
pub use crate::enums::VideoAdjustOption as libvlc_video_adjust_option;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_video_logo_option_t {
    libvlc_logo_enable,
    libvlc_logo_file,
    libvlc_logo_x,
    libvlc_logo_y,
    libvlc_logo_delay,
    libvlc_logo_repeat,
    libvlc_logo_opacity,
    libvlc_logo_position
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_audio_output_device_types_t {
    libvlc_AudioOutputDevice_Error  = -1,
    libvlc_AudioOutputDevice_Mono   =  1,
    libvlc_AudioOutputDevice_Stereo =  2,
    libvlc_AudioOutputDevice_2F2R   =  4,
    libvlc_AudioOutputDevice_3F2R   =  5,
    libvlc_AudioOutputDevice_5_1    =  6,
    libvlc_AudioOutputDevice_6_1    =  7,
    libvlc_AudioOutputDevice_7_1    =  8,
    libvlc_AudioOutputDevice_SPDIF  = 10
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_audio_output_channel_t {
    libvlc_AudioChannel_Error   = -1,
    libvlc_AudioChannel_Stereo  =  1,
    libvlc_AudioChannel_RStereo =  2,
    libvlc_AudioChannel_Left    =  3,
    libvlc_AudioChannel_Right   =  4,
    libvlc_AudioChannel_Dolbys  =  5
}

pub type libvlc_video_lock_cb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>;
pub type libvlc_video_unlock_cb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *const *mut c_void)>;
pub type libvlc_video_display_cb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>;
pub type libvlc_video_format_cb = Option<unsafe extern "C" fn(*mut *mut c_void, *mut c_char, *mut c_uint, *mut c_uint, *mut c_uint, *mut c_uint)>;
pub type libvlc_video_cleanup_cb = Option<unsafe extern "C" fn(*mut c_void)>;
pub type libvlc_audio_play_cb = Option<unsafe extern "C" fn(*mut c_void, *const c_void, c_uint, i64)>;
pub type libvlc_audio_pause_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_resume_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_flush_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_drain_cb = Option<unsafe extern "C" fn(*mut c_void)>;
pub type libvlc_audio_set_volume_cb = Option<unsafe extern "C" fn(*mut c_void, c_float, c_bool)>;
pub type libvlc_audio_setup_cb = Option<unsafe extern "C" fn(*mut *mut c_void, *mut c_char, *mut c_uint, *mut c_uint)>;
pub type libvlc_audio_cleanup_cb = Option<unsafe extern "C" fn(*mut c_void)>;

// From libvlc_events.h
pub use crate::enums::EventType as libvlc_event_e;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_event_t {
    pub _type: c_int,
    pub p_obj: *mut c_void,
    pub u: libvlc_event_t_types::u,
}

pub mod libvlc_event_t_types {
    use super::*;
    use libc::{c_int, c_char, c_float};
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub media_meta_changed: media_meta_changed,
        pub media_subitem_added: media_subitem_added,
        pub media_duration_changed: media_duration_changed,
        pub media_parsed_changed: media_parsed_changed,
        pub media_freed: media_freed,
        pub media_state_changed: media_state_changed,
        pub media_subitemtree_added: media_subitemtree_added,
        pub media_player_buffering: media_player_buffering,
        pub media_player_position_changed: media_player_position_changed,
        pub media_player_time_changed: media_player_time_changed,
        pub media_player_title_changed: media_player_title_changed,
        pub media_player_seekable_changed: media_player_seekable_changed,
        pub media_player_pausable_changed: media_player_pausable_changed,
        pub media_player_scrambled_changed: media_player_scrambled_changed,
        pub media_player_vout: media_player_vout,
        pub media_list_item_added: media_list_item_added,
        pub media_list_will_add_item: media_list_will_add_item,
        pub media_list_item_deleted: media_list_item_deleted,
        pub media_list_will_delete_item: media_list_will_delete_item,
        pub media_list_player_next_item_set: media_list_player_next_item_set,
        pub media_player_snapshot_taken: media_player_snapshot_taken,
        pub media_player_length_changed: media_player_length_changed,
        pub vlm_media_event: vlm_media_event,
        pub media_player_media_changed: media_player_media_changed,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_meta_changed {
        pub meta_type: libvlc_meta_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_subitem_added {
        pub new_child: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_duration_changed {
        pub new_duration: i64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_parsed_changed {
        pub new_status: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_freed {
        pub md: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_state_changed {
        pub new_state: libvlc_state_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_subitemtree_added {
        pub item: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_buffering {
        pub new_cache: c_float,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_position_changed {
        pub new_position: c_float,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_time_changed {
        pub new_time: libvlc_time_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_title_changed {
        pub new_titie: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_seekable_changed {
        pub new_seekable: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_pausable_changed {
        pub new_pausable: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_scrambled_changed {
        pub new_scrambled: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_vout {
        pub new_count: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_item_added {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_will_add_item {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_item_deleted {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_will_delete_item {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_player_next_item_set {
        pub item: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_snapshot_taken {
        pub psz_filename: *mut c_char,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_length_changed {
        pub new_length: libvlc_time_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct vlm_media_event {
        pub psz_media_name: *mut c_char,
        pub psz_instance_name: *mut c_char,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_media_changed {
        pub new_media: *mut libvlc_media_t,
    }
}

pub enum libvlc_media_list_t {}
pub enum libvlc_media_library_t {}
pub enum libvlc_media_discoverer_t {}

pub struct VlcDll {
    pub lib: Library,
    pub libvlc_errmsg: extern "C" fn() -> *const c_char,
    pub libvlc_clearerr: extern "C" fn(),
    pub libvlc_new: extern "C" fn(_: c_int, _: *const *const c_char) -> *mut libvlc_instance_t,
    pub libvlc_release: extern "C" fn(_: *mut libvlc_instance_t),
    pub libvlc_retain: extern "C" fn(_: *mut libvlc_instance_t),
    pub libvlc_add_intf: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int,
    pub libvlc_set_exit_handler: extern "C" fn(_: *mut libvlc_instance_t, _: extern "C" fn(*mut c_void), _: *mut c_void),
    pub libvlc_wait: extern "C" fn(_: *mut libvlc_instance_t),
    pub libvlc_set_user_agent: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char),
    pub libvlc_set_app_id: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char),
    pub libvlc_get_version: extern "C" fn() -> *const c_char,
    pub libvlc_get_compiler: extern "C" fn() -> *const c_char,
    pub libvlc_get_changeset: extern "C" fn() -> *const c_char,
    pub libvlc_free: extern "C" fn(_: *mut c_void),
    pub libvlc_event_attach: extern "C" fn(_: *mut libvlc_event_manager_t, _: libvlc_event_type_t, _: libvlc_callback_t, _: *mut c_void) -> c_int,
    pub libvlc_event_type_name: extern "C" fn(_: libvlc_event_type_t) -> *const c_char,
    pub libvlc_log_get_context: extern "C" fn(_: *const libvlc_log_t, _: *const *const c_char, _: *const *const c_char, _: *mut c_uint),
    pub libvlc_log_get_object: extern "C" fn(_: *const libvlc_log_t, _: *const *const c_char, _: *const *const c_char, _: *mut uintptr_t),
    pub libvlc_log_unset: extern "C" fn(_: *mut libvlc_instance_t),
    pub libvlc_log_set: extern "C" fn(_: *mut libvlc_instance_t, _: libvlc_log_cb, _: *mut c_void),
    pub libvlc_log_set_file: extern "C" fn(_: *mut libvlc_instance_t, _: *mut FILE),
    pub libvlc_module_description_list_release: extern "C" fn(_: *mut libvlc_module_description_t),
    pub libvlc_audio_filter_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_module_description_t,
    pub libvlc_video_filter_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_module_description_t,
    pub libvlc_clock: extern "C" fn() -> i64,
    pub libvlc_media_new_location: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_t,
    pub libvlc_media_new_path: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_t,
    pub libvlc_media_new_fd: extern "C" fn(_: *mut libvlc_instance_t, _: c_int) -> *mut libvlc_media_t,
    pub libvlc_media_add_option: extern "C" fn(_: *mut libvlc_media_t, _: *const c_char),
    pub libvlc_media_add_option_flag: extern "C" fn(_: *mut libvlc_media_t, _: *const c_char, _: c_uint),
    pub libvlc_media_retain: extern "C" fn(_: *mut libvlc_media_t),
    pub libvlc_media_release: extern "C" fn(_: *mut libvlc_media_t),
    pub libvlc_media_get_mrl: extern "C" fn(_: *mut libvlc_media_t) -> *mut c_char,
    pub libvlc_media_duplicate: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_t,
    pub libvlc_media_get_meta: extern "C" fn(_: *mut libvlc_media_t, _: libvlc_meta_t) -> *mut c_char,
    pub libvlc_media_set_meta: extern "C" fn(_: *mut libvlc_media_t, _: libvlc_meta_t, _: *const c_char),
    pub libvlc_media_save_meta: extern "C" fn(_: *mut libvlc_media_t) -> c_int,
    pub libvlc_media_get_state: extern "C" fn(_: *mut libvlc_media_t) -> libvlc_state_t,
    pub libvlc_media_get_stats: extern "C" fn(_: *mut libvlc_media_t, _: *mut libvlc_media_stats_t) -> c_int,
    pub libvlc_media_subitems: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_list_t,
    pub libvlc_media_event_manager: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_event_manager_t,
    pub libvlc_media_get_duration: extern "C" fn(_: *mut libvlc_media_t) -> libvlc_time_t,
    pub libvlc_media_parse: extern "C" fn(_: *mut libvlc_media_t),
    pub libvlc_media_parse_async: extern "C" fn(_: *mut libvlc_media_t),
    pub libvlc_media_is_parsed: extern "C" fn(_: *mut libvlc_media_t) -> c_int,
    pub libvlc_media_set_user_data: extern "C" fn(_: *mut libvlc_media_t, _: *mut c_void),
    pub libvlc_media_get_user_data: extern "C" fn(_: *mut libvlc_media_t) -> *mut c_void,
    pub libvlc_media_tracks_get: extern "C" fn(_: *mut libvlc_media_t, _: *mut *mut *mut libvlc_media_track_t) -> c_uint,
    pub libvlc_media_tracks_release: extern "C" fn(_: *mut *mut libvlc_media_track_t, _: c_uint),
    pub libvlc_media_player_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_player_t,
    pub libvlc_media_player_new_from_media: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_player_t,
    pub libvlc_media_player_release: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_retain: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_set_media: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut libvlc_media_t),
    pub libvlc_media_player_get_media: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_media_t,
    pub libvlc_media_player_event_manager: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_event_manager_t,
    pub libvlc_media_player_is_playing: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_play: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_set_pause: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_media_player_pause: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_stop: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_video_set_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_video_lock_cb, _: libvlc_video_unlock_cb, _: libvlc_video_display_cb, _: *mut c_void),
    pub libvlc_video_set_format: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: c_uint, _: c_uint, _: c_uint),
    pub libvlc_video_set_format_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_video_format_cb, _: libvlc_video_cleanup_cb),
    pub libvlc_media_player_set_nsobject: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut c_void),
    pub libvlc_media_player_get_nsobject: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_void,
    pub libvlc_media_player_set_xwindow: extern "C" fn(_: *mut libvlc_media_player_t, _: u32),
    pub libvlc_media_player_get_xwindow: extern "C" fn(_: *mut libvlc_media_player_t) -> u32,
    pub libvlc_media_player_set_hwnd: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut c_void),
    pub libvlc_media_player_get_hwnd: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_void,
    pub libvlc_audio_set_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_play_cb, _: libvlc_audio_pause_cb, _: libvlc_audio_resume_cb, _: libvlc_audio_flush_cb, _: libvlc_audio_drain_cb, _: *mut c_void),
    pub libvlc_audio_set_volume_callback: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_set_volume_cb),
    pub libvlc_audio_set_format_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_setup_cb, _: libvlc_audio_cleanup_cb),
    pub libvlc_audio_set_format: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: c_uint, _: c_uint),
    pub libvlc_media_player_get_length: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_time_t,
    pub libvlc_media_player_get_time: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_time_t,
    pub libvlc_media_player_set_time: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_time_t),
    pub libvlc_media_player_get_position: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float,
    pub libvlc_media_player_set_position: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float),
    pub libvlc_media_player_set_chapter: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_media_player_get_chapter: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_get_chapter_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_will_play: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_set_title: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_media_player_get_chapter_count_for_title: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_media_player_get_title: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_get_title_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_previous_chapter: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_next_chapter: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_get_rate: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float,
    pub libvlc_media_player_set_rate: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float) -> c_int,
    pub libvlc_media_player_get_state: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_state_t,
    pub libvlc_media_player_get_fps: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float,
    pub libvlc_media_player_has_vout: extern "C" fn(_: *mut libvlc_media_player_t) -> c_uint,
    pub libvlc_media_player_is_seekable: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_can_pause: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_program_scrambled: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_media_player_next_frame: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_media_player_navigate: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint),
    pub libvlc_media_player_set_video_title_display: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_position_t, _: c_uint),
    pub libvlc_track_description_list_release: extern "C" fn(_: *mut libvlc_track_description_t),
    pub libvlc_toggle_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_set_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_get_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_set_key_input: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint),
    pub libvlc_video_set_mouse_input: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint),
    pub libvlc_video_get_size: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *mut c_uint, _: *mut c_uint) -> c_int,
    pub libvlc_video_get_cursor: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *mut c_int, _: *mut c_int) -> c_int,
    pub libvlc_video_get_scale: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float,
    pub libvlc_video_set_scale: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float),
    pub libvlc_video_get_aspect_ratio: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_char,
    pub libvlc_video_set_aspect_ratio: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char),
    pub libvlc_video_get_spu: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_get_spu_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_get_spu_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t,
    pub libvlc_video_set_spu: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_video_set_subtitle_file: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) -> c_int,
    pub libvlc_video_get_spu_delay: extern "C" fn(_: *mut libvlc_media_player_t) -> i64,
    pub libvlc_video_set_spu_delay: extern "C" fn(_: *mut libvlc_media_player_t, _: i64) -> c_int,
    pub libvlc_video_get_title_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t,
    pub libvlc_video_get_chapter_description: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> *mut libvlc_track_description_t,
    pub libvlc_video_get_crop_geometry: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_char,
    pub libvlc_video_set_crop_geometry: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char),
    pub libvlc_video_get_teletext: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_set_teletext: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_toggle_teletext: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_video_get_track_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_get_track_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t,
    pub libvlc_video_get_track: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_video_set_track: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_video_take_snapshot: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char, _: c_uint, _: c_uint) -> c_int,
    pub libvlc_video_set_deinterlace: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char),
    pub libvlc_video_get_marquee_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int,
    pub libvlc_video_get_marquee_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> *mut c_char,
    pub libvlc_video_set_marquee_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int),
    pub libvlc_video_set_marquee_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char),
    pub libvlc_video_get_logo_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int,
    pub libvlc_video_set_logo_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int),
    pub libvlc_video_set_logo_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char),
    pub libvlc_video_get_adjust_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int,
    pub libvlc_video_set_adjust_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int),
    pub libvlc_video_get_adjust_float: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_float,
    pub libvlc_video_set_adjust_float: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_float),
    pub libvlc_audio_output_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_audio_output_t,
    pub libvlc_audio_output_list_release: extern "C" fn(_: *mut libvlc_audio_output_t),
    pub libvlc_audio_output_set: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) -> c_int,
    pub libvlc_audio_output_device_enum: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_audio_output_device_t,
    pub libvlc_audio_output_device_list_get: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_audio_output_device_t,
    pub libvlc_audio_output_device_list_release: extern "C" fn(_: *mut libvlc_audio_output_device_t),
    pub libvlc_audio_output_device_set: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: *const c_char),
    pub libvlc_audio_toggle_mute: extern "C" fn(_: *mut libvlc_media_player_t),
    pub libvlc_audio_get_mute: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_audio_set_mute: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int),
    pub libvlc_audio_get_volume: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_audio_set_volume: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_audio_get_track_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_audio_get_track_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t,
    pub libvlc_audio_get_track: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_audio_set_track: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_audio_get_channel: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int,
    pub libvlc_audio_set_channel: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int,
    pub libvlc_audio_get_delay: extern "C" fn(_: *mut libvlc_media_player_t) -> i64,
    pub libvlc_audio_set_delay: extern "C" fn(_: *mut libvlc_media_player_t, _: i64) -> c_int,
    pub libvlc_audio_equalizer_get_preset_count: extern "C" fn() -> c_uint,
    pub libvlc_audio_equalizer_get_preset_name: extern "C" fn(_: c_uint) -> *const c_char,
    pub libvlc_audio_equalizer_get_band_count: extern "C" fn() -> c_uint,
    pub libvlc_audio_equalizer_get_band_frequency: extern "C" fn(_: c_uint) -> c_float,
    pub libvlc_audio_equalizer_new: extern "C" fn() -> *mut libvlc_equalizer_t,
    pub libvlc_audio_equalizer_new_from_preset: extern "C" fn(_: c_uint) -> *mut libvlc_equalizer_t,
    pub libvlc_audio_equalizer_release: extern "C" fn(_: *mut libvlc_equalizer_t),
    pub libvlc_audio_equalizer_set_preamp: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_float) -> c_int,
    pub libvlc_audio_equalizer_get_preamp: extern "C" fn(_: *mut libvlc_equalizer_t) -> c_float,
    pub libvlc_audio_equalizer_set_amp_at_index: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_float, _: c_uint) -> c_int,
    pub libvlc_audio_equalizer_get_amp_at_index: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_uint) -> c_float,
    pub libvlc_media_player_set_equalizer: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut libvlc_equalizer_t) -> c_int,
    pub libvlc_media_list_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_list_t,
    pub libvlc_media_list_release: extern "C" fn(_: *mut libvlc_media_list_t),
    pub libvlc_media_list_retain: extern "C" fn(_: *mut libvlc_media_list_t),
    pub libvlc_media_list_set_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t),
    pub libvlc_media_list_media: extern "C" fn(_: *mut libvlc_media_list_t) -> *mut libvlc_media_t,
    pub libvlc_media_list_add_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t) -> c_int,
    pub libvlc_media_list_insert_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t, _: c_int) -> c_int,
    pub libvlc_media_list_remove_index: extern "C" fn(_: *mut libvlc_media_list_t, _: c_int) -> c_int,
    pub libvlc_media_list_count: extern "C" fn(_: *mut libvlc_media_list_t) -> c_int,
    pub libvlc_media_list_item_at_index: extern "C" fn(_: *mut libvlc_media_list_t, _: c_int) -> *mut libvlc_media_t,
    pub libvlc_media_list_index_of_item: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t) -> c_int,
    pub libvlc_media_list_is_readonly: extern "C" fn(_: *mut libvlc_media_list_t) -> c_int,
    pub libvlc_media_list_lock: extern "C" fn(_: *mut libvlc_media_list_t),
    pub libvlc_media_list_unlock: extern "C" fn(_: *mut libvlc_media_list_t),
    pub libvlc_media_list_event_manager: extern "C" fn(_: *mut libvlc_media_list_t) -> *mut libvlc_event_manager_t,
    pub libvlc_media_library_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_library_t,
    pub libvlc_media_library_release: extern "C" fn(_: *mut libvlc_media_library_t),
    pub libvlc_media_library_retain: extern "C" fn(_: *mut libvlc_media_library_t),
    pub libvlc_media_library_load: extern "C" fn(_: *mut libvlc_media_library_t) -> c_int,
    pub libvlc_media_library_media_list: extern "C" fn(_: *mut libvlc_media_library_t) -> *mut libvlc_media_list_t,
    pub libvlc_media_discoverer_new_from_name: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_discoverer_t,
    pub libvlc_media_discoverer_release: extern "C" fn(_: *mut libvlc_media_discoverer_t),
    pub libvlc_media_discoverer_localized_name: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut c_char,
    pub libvlc_media_discoverer_media_list: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut libvlc_media_list_t,
    pub libvlc_media_discoverer_event_manager: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut libvlc_event_manager_t,
    pub libvlc_media_discoverer_is_running: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> c_int,
    pub libvlc_vlm_release: extern "C" fn(_: *mut libvlc_instance_t),
    pub libvlc_vlm_add_broadcast: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: c_int) -> c_int,
    pub libvlc_vlm_add_vod: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: *const c_char) -> c_int,
    pub libvlc_vlm_del_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int,
    pub libvlc_vlm_set_enabled: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int,
    pub libvlc_vlm_set_output: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int,
    pub libvlc_vlm_set_input: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int,
    pub libvlc_vlm_add_input: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int,
    pub libvlc_vlm_set_loop: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int,
    pub libvlc_vlm_set_mux: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int,
    pub libvlc_vlm_change_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: c_int) -> c_int,
    pub libvlc_vlm_play_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int,
    pub libvlc_vlm_stop_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int,
    pub libvlc_vlm_pause_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int,
    pub libvlc_vlm_seek_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_float) -> c_int,
    pub libvlc_vlm_show_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *const c_char,
    pub libvlc_vlm_get_media_instance_position: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_float,
    pub libvlc_vlm_get_media_instance_time: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int,
    pub libvlc_vlm_get_media_instance_length: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int,
    pub libvlc_vlm_get_media_instance_rate: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int,
    pub libvlc_vlm_get_event_manager: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_event_manager_t,
}

fn initialize_library(path: &std::path::Path) -> Option<VlcDll> {
    use std::mem::transmute;
    let lib = Library::new(path)?;
    unsafe {
        let libvlc_errmsg: extern "C" fn() -> *const c_char = transmute(lib.get(b"libvlc_errmsg")?);
        let libvlc_clearerr: extern "C" fn() = transmute(lib.get(b"libvlc_clearerr")?);
        let libvlc_new: extern "C" fn(_: c_int, _: *const *const c_char) -> *mut libvlc_instance_t = transmute(lib.get(b"libvlc_new")?);
        let libvlc_release: extern "C" fn(_: *mut libvlc_instance_t) = transmute(lib.get(b"libvlc_release")?);
        let libvlc_retain: extern "C" fn(_: *mut libvlc_instance_t) = transmute(lib.get(b"libvlc_retain")?);
        let libvlc_add_intf: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_add_intf")?);
        let libvlc_set_exit_handler: extern "C" fn(_: *mut libvlc_instance_t, _: extern "C" fn(*mut c_void), _: *mut c_void) = transmute(lib.get(b"libvlc_set_exit_handler")?);
        let libvlc_wait: extern "C" fn(_: *mut libvlc_instance_t) = transmute(lib.get(b"libvlc_wait")?);
        let libvlc_set_user_agent: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) = transmute(lib.get(b"libvlc_set_user_agent")?);
        let libvlc_set_app_id: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char) = transmute(lib.get(b"libvlc_set_app_id")?);
        let libvlc_get_version: extern "C" fn() -> *const c_char = transmute(lib.get(b"libvlc_get_version")?);
        let libvlc_get_compiler: extern "C" fn() -> *const c_char = transmute(lib.get(b"libvlc_get_compiler")?);
        let libvlc_get_changeset: extern "C" fn() -> *const c_char = transmute(lib.get(b"libvlc_get_changeset")?);
        let libvlc_free: extern "C" fn(_: *mut c_void) = transmute(lib.get(b"libvlc_free")?);
        let libvlc_event_attach: extern "C" fn(_: *mut libvlc_event_manager_t, _: libvlc_event_type_t, _: libvlc_callback_t, _: *mut c_void) -> c_int = transmute(lib.get(b"libvlc_event_attach")?);
        let libvlc_event_type_name: extern "C" fn(_: libvlc_event_type_t) -> *const c_char = transmute(lib.get(b"libvlc_event_type_name")?);
        let libvlc_log_get_context: extern "C" fn(_: *const libvlc_log_t, _: *const *const c_char, _: *const *const c_char, _: *mut c_uint) = transmute(lib.get(b"libvlc_log_get_context")?);
        let libvlc_log_get_object: extern "C" fn(_: *const libvlc_log_t, _: *const *const c_char, _: *const *const c_char, _: *mut uintptr_t) = transmute(lib.get(b"libvlc_log_get_object")?);
        let libvlc_log_unset: extern "C" fn(_: *mut libvlc_instance_t) = transmute(lib.get(b"libvlc_log_unset")?);
        let libvlc_log_set: extern "C" fn(_: *mut libvlc_instance_t, _: libvlc_log_cb, _: *mut c_void) = transmute(lib.get(b"libvlc_log_set")?);
        let libvlc_log_set_file: extern "C" fn(_: *mut libvlc_instance_t, _: *mut FILE) = transmute(lib.get(b"libvlc_log_set_file")?);
        let libvlc_module_description_list_release: extern "C" fn(_: *mut libvlc_module_description_t) = transmute(lib.get(b"libvlc_module_description_list_release")?);
        let libvlc_audio_filter_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_module_description_t = transmute(lib.get(b"libvlc_audio_filter_list_get")?);
        let libvlc_video_filter_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_module_description_t = transmute(lib.get(b"libvlc_video_filter_list_get")?);
        let libvlc_clock: extern "C" fn() -> i64 = transmute(lib.get(b"libvlc_clock")?);
        let libvlc_media_new_location: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_new_location")?);
        let libvlc_media_new_path: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_new_path")?);
        let libvlc_media_new_fd: extern "C" fn(_: *mut libvlc_instance_t, _: c_int) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_new_fd")?);
        let libvlc_media_add_option: extern "C" fn(_: *mut libvlc_media_t, _: *const c_char) = transmute(lib.get(b"libvlc_media_add_option")?);
        let libvlc_media_add_option_flag: extern "C" fn(_: *mut libvlc_media_t, _: *const c_char, _: c_uint) = transmute(lib.get(b"libvlc_media_add_option_flag")?);
        let libvlc_media_retain: extern "C" fn(_: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_retain")?);
        let libvlc_media_release: extern "C" fn(_: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_release")?);
        let libvlc_media_get_mrl: extern "C" fn(_: *mut libvlc_media_t) -> *mut c_char = transmute(lib.get(b"libvlc_media_get_mrl")?);
        let libvlc_media_duplicate: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_duplicate")?);
        let libvlc_media_get_meta: extern "C" fn(_: *mut libvlc_media_t, _: libvlc_meta_t) -> *mut c_char = transmute(lib.get(b"libvlc_media_get_meta")?);
        let libvlc_media_set_meta: extern "C" fn(_: *mut libvlc_media_t, _: libvlc_meta_t, _: *const c_char) = transmute(lib.get(b"libvlc_media_set_meta")?);
        let libvlc_media_save_meta: extern "C" fn(_: *mut libvlc_media_t) -> c_int = transmute(lib.get(b"libvlc_media_save_meta")?);
        let libvlc_media_get_state: extern "C" fn(_: *mut libvlc_media_t) -> libvlc_state_t = transmute(lib.get(b"libvlc_media_get_state")?);
        let libvlc_media_get_stats: extern "C" fn(_: *mut libvlc_media_t, _: *mut libvlc_media_stats_t) -> c_int = transmute(lib.get(b"libvlc_media_get_stats")?);
        let libvlc_media_subitems: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_list_t = transmute(lib.get(b"libvlc_media_subitems")?);
        let libvlc_media_event_manager: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_event_manager_t = transmute(lib.get(b"libvlc_media_event_manager")?);
        let libvlc_media_get_duration: extern "C" fn(_: *mut libvlc_media_t) -> libvlc_time_t = transmute(lib.get(b"libvlc_media_get_duration")?);
        let libvlc_media_parse: extern "C" fn(_: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_parse")?);
        let libvlc_media_parse_async: extern "C" fn(_: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_parse_async")?);
        let libvlc_media_is_parsed: extern "C" fn(_: *mut libvlc_media_t) -> c_int = transmute(lib.get(b"libvlc_media_is_parsed")?);
        let libvlc_media_set_user_data: extern "C" fn(_: *mut libvlc_media_t, _: *mut c_void) = transmute(lib.get(b"libvlc_media_set_user_data")?);
        let libvlc_media_get_user_data: extern "C" fn(_: *mut libvlc_media_t) -> *mut c_void = transmute(lib.get(b"libvlc_media_get_user_data")?);
        let libvlc_media_tracks_get: extern "C" fn(_: *mut libvlc_media_t, _: *mut *mut *mut libvlc_media_track_t) -> c_uint = transmute(lib.get(b"libvlc_media_tracks_get")?);
        let libvlc_media_tracks_release: extern "C" fn(_: *mut *mut libvlc_media_track_t, _: c_uint) = transmute(lib.get(b"libvlc_media_tracks_release")?);
        let libvlc_media_player_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_player_t = transmute(lib.get(b"libvlc_media_player_new")?);
        let libvlc_media_player_new_from_media: extern "C" fn(_: *mut libvlc_media_t) -> *mut libvlc_media_player_t = transmute(lib.get(b"libvlc_media_player_new_from_media")?);
        let libvlc_media_player_release: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_release")?);
        let libvlc_media_player_retain: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_retain")?);
        let libvlc_media_player_set_media: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_player_set_media")?);
        let libvlc_media_player_get_media: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_player_get_media")?);
        let libvlc_media_player_event_manager: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_event_manager_t = transmute(lib.get(b"libvlc_media_player_event_manager")?);
        let libvlc_media_player_is_playing: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_is_playing")?);
        let libvlc_media_player_play: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_play")?);
        let libvlc_media_player_set_pause: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_media_player_set_pause")?);
        let libvlc_media_player_pause: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_pause")?);
        let libvlc_media_player_stop: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_stop")?);
        let libvlc_video_set_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_video_lock_cb, _: libvlc_video_unlock_cb, _: libvlc_video_display_cb, _: *mut c_void) = transmute(lib.get(b"libvlc_video_set_callbacks")?);
        let libvlc_video_set_format: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: c_uint, _: c_uint, _: c_uint) = transmute(lib.get(b"libvlc_video_set_format")?);
        let libvlc_video_set_format_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_video_format_cb, _: libvlc_video_cleanup_cb) = transmute(lib.get(b"libvlc_video_set_format_callbacks")?);
        let libvlc_media_player_set_nsobject: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut c_void) = transmute(lib.get(b"libvlc_media_player_set_nsobject")?);
        let libvlc_media_player_get_nsobject: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_void = transmute(lib.get(b"libvlc_media_player_get_nsobject")?);
        let libvlc_media_player_set_xwindow: extern "C" fn(_: *mut libvlc_media_player_t, _: u32) = transmute(lib.get(b"libvlc_media_player_set_xwindow")?);
        let libvlc_media_player_get_xwindow: extern "C" fn(_: *mut libvlc_media_player_t) -> u32 = transmute(lib.get(b"libvlc_media_player_get_xwindow")?);
        let libvlc_media_player_set_hwnd: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut c_void) = transmute(lib.get(b"libvlc_media_player_set_hwnd")?);
        let libvlc_media_player_get_hwnd: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_void = transmute(lib.get(b"libvlc_media_player_get_hwnd")?);
        let libvlc_audio_set_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_play_cb, _: libvlc_audio_pause_cb, _: libvlc_audio_resume_cb, _: libvlc_audio_flush_cb, _: libvlc_audio_drain_cb, _: *mut c_void) = transmute(lib.get(b"libvlc_audio_set_callbacks")?);
        let libvlc_audio_set_volume_callback: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_set_volume_cb) = transmute(lib.get(b"libvlc_audio_set_volume_callback")?);
        let libvlc_audio_set_format_callbacks: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_audio_setup_cb, _: libvlc_audio_cleanup_cb) = transmute(lib.get(b"libvlc_audio_set_format_callbacks")?);
        let libvlc_audio_set_format: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: c_uint, _: c_uint) = transmute(lib.get(b"libvlc_audio_set_format")?);
        let libvlc_media_player_get_length: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_time_t = transmute(lib.get(b"libvlc_media_player_get_length")?);
        let libvlc_media_player_get_time: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_time_t = transmute(lib.get(b"libvlc_media_player_get_time")?);
        let libvlc_media_player_set_time: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_time_t) = transmute(lib.get(b"libvlc_media_player_set_time")?);
        let libvlc_media_player_get_position: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float = transmute(lib.get(b"libvlc_media_player_get_position")?);
        let libvlc_media_player_set_position: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float) = transmute(lib.get(b"libvlc_media_player_set_position")?);
        let libvlc_media_player_set_chapter: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_media_player_set_chapter")?);
        let libvlc_media_player_get_chapter: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_get_chapter")?);
        let libvlc_media_player_get_chapter_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_get_chapter_count")?);
        let libvlc_media_player_will_play: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_will_play")?);
        let libvlc_media_player_set_title: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_media_player_set_title")?);
        let libvlc_media_player_get_chapter_count_for_title: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_media_player_get_chapter_count_for_title")?);
        let libvlc_media_player_get_title: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_get_title")?);
        let libvlc_media_player_get_title_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_get_title_count")?);
        let libvlc_media_player_previous_chapter: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_previous_chapter")?);
        let libvlc_media_player_next_chapter: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_next_chapter")?);
        let libvlc_media_player_get_rate: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float = transmute(lib.get(b"libvlc_media_player_get_rate")?);
        let libvlc_media_player_set_rate: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float) -> c_int = transmute(lib.get(b"libvlc_media_player_set_rate")?);
        let libvlc_media_player_get_state: extern "C" fn(_: *mut libvlc_media_player_t) -> libvlc_state_t = transmute(lib.get(b"libvlc_media_player_get_state")?);
        let libvlc_media_player_get_fps: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float = transmute(lib.get(b"libvlc_media_player_get_fps")?);
        let libvlc_media_player_has_vout: extern "C" fn(_: *mut libvlc_media_player_t) -> c_uint = transmute(lib.get(b"libvlc_media_player_has_vout")?);
        let libvlc_media_player_is_seekable: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_is_seekable")?);
        let libvlc_media_player_can_pause: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_can_pause")?);
        let libvlc_media_player_program_scrambled: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_media_player_program_scrambled")?);
        let libvlc_media_player_next_frame: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_media_player_next_frame")?);
        let libvlc_media_player_navigate: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) = transmute(lib.get(b"libvlc_media_player_navigate")?);
        let libvlc_media_player_set_video_title_display: extern "C" fn(_: *mut libvlc_media_player_t, _: libvlc_position_t, _: c_uint) = transmute(lib.get(b"libvlc_media_player_set_video_title_display")?);
        let libvlc_track_description_list_release: extern "C" fn(_: *mut libvlc_track_description_t) = transmute(lib.get(b"libvlc_track_description_list_release")?);
        let libvlc_toggle_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_toggle_fullscreen")?);
        let libvlc_set_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_set_fullscreen")?);
        let libvlc_get_fullscreen: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_get_fullscreen")?);
        let libvlc_video_set_key_input: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) = transmute(lib.get(b"libvlc_video_set_key_input")?);
        let libvlc_video_set_mouse_input: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) = transmute(lib.get(b"libvlc_video_set_mouse_input")?);
        let libvlc_video_get_size: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *mut c_uint, _: *mut c_uint) -> c_int = transmute(lib.get(b"libvlc_video_get_size")?);
        let libvlc_video_get_cursor: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *mut c_int, _: *mut c_int) -> c_int = transmute(lib.get(b"libvlc_video_get_cursor")?);
        let libvlc_video_get_scale: extern "C" fn(_: *mut libvlc_media_player_t) -> c_float = transmute(lib.get(b"libvlc_video_get_scale")?);
        let libvlc_video_set_scale: extern "C" fn(_: *mut libvlc_media_player_t, _: c_float) = transmute(lib.get(b"libvlc_video_set_scale")?);
        let libvlc_video_get_aspect_ratio: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_char = transmute(lib.get(b"libvlc_video_get_aspect_ratio")?);
        let libvlc_video_set_aspect_ratio: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) = transmute(lib.get(b"libvlc_video_set_aspect_ratio")?);
        let libvlc_video_get_spu: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_video_get_spu")?);
        let libvlc_video_get_spu_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_video_get_spu_count")?);
        let libvlc_video_get_spu_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t = transmute(lib.get(b"libvlc_video_get_spu_description")?);
        let libvlc_video_set_spu: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_video_set_spu")?);
        let libvlc_video_set_subtitle_file: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_video_set_subtitle_file")?);
        let libvlc_video_get_spu_delay: extern "C" fn(_: *mut libvlc_media_player_t) -> i64 = transmute(lib.get(b"libvlc_video_get_spu_delay")?);
        let libvlc_video_set_spu_delay: extern "C" fn(_: *mut libvlc_media_player_t, _: i64) -> c_int = transmute(lib.get(b"libvlc_video_set_spu_delay")?);
        let libvlc_video_get_title_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t = transmute(lib.get(b"libvlc_video_get_title_description")?);
        let libvlc_video_get_chapter_description: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> *mut libvlc_track_description_t = transmute(lib.get(b"libvlc_video_get_chapter_description")?);
        let libvlc_video_get_crop_geometry: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut c_char = transmute(lib.get(b"libvlc_video_get_crop_geometry")?);
        let libvlc_video_set_crop_geometry: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) = transmute(lib.get(b"libvlc_video_set_crop_geometry")?);
        let libvlc_video_get_teletext: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_video_get_teletext")?);
        let libvlc_video_set_teletext: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_video_set_teletext")?);
        let libvlc_toggle_teletext: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_toggle_teletext")?);
        let libvlc_video_get_track_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_video_get_track_count")?);
        let libvlc_video_get_track_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t = transmute(lib.get(b"libvlc_video_get_track_description")?);
        let libvlc_video_get_track: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_video_get_track")?);
        let libvlc_video_set_track: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_video_set_track")?);
        let libvlc_video_take_snapshot: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char, _: c_uint, _: c_uint) -> c_int = transmute(lib.get(b"libvlc_video_take_snapshot")?);
        let libvlc_video_set_deinterlace: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) = transmute(lib.get(b"libvlc_video_set_deinterlace")?);
        let libvlc_video_get_marquee_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int = transmute(lib.get(b"libvlc_video_get_marquee_int")?);
        let libvlc_video_get_marquee_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> *mut c_char = transmute(lib.get(b"libvlc_video_get_marquee_string")?);
        let libvlc_video_set_marquee_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int) = transmute(lib.get(b"libvlc_video_set_marquee_int")?);
        let libvlc_video_set_marquee_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char) = transmute(lib.get(b"libvlc_video_set_marquee_string")?);
        let libvlc_video_get_logo_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int = transmute(lib.get(b"libvlc_video_get_logo_int")?);
        let libvlc_video_set_logo_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int) = transmute(lib.get(b"libvlc_video_set_logo_int")?);
        let libvlc_video_set_logo_string: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: *const c_char) = transmute(lib.get(b"libvlc_video_set_logo_string")?);
        let libvlc_video_get_adjust_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_int = transmute(lib.get(b"libvlc_video_get_adjust_int")?);
        let libvlc_video_set_adjust_int: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_int) = transmute(lib.get(b"libvlc_video_set_adjust_int")?);
        let libvlc_video_get_adjust_float: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint) -> c_float = transmute(lib.get(b"libvlc_video_get_adjust_float")?);
        let libvlc_video_set_adjust_float: extern "C" fn(_: *mut libvlc_media_player_t, _: c_uint, _: c_float) = transmute(lib.get(b"libvlc_video_set_adjust_float")?);
        let libvlc_audio_output_list_get: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_audio_output_t = transmute(lib.get(b"libvlc_audio_output_list_get")?);
        let libvlc_audio_output_list_release: extern "C" fn(_: *mut libvlc_audio_output_t) = transmute(lib.get(b"libvlc_audio_output_list_release")?);
        let libvlc_audio_output_set: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_audio_output_set")?);
        let libvlc_audio_output_device_enum: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_audio_output_device_t = transmute(lib.get(b"libvlc_audio_output_device_enum")?);
        let libvlc_audio_output_device_list_get: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_audio_output_device_t = transmute(lib.get(b"libvlc_audio_output_device_list_get")?);
        let libvlc_audio_output_device_list_release: extern "C" fn(_: *mut libvlc_audio_output_device_t) = transmute(lib.get(b"libvlc_audio_output_device_list_release")?);
        let libvlc_audio_output_device_set: extern "C" fn(_: *mut libvlc_media_player_t, _: *const c_char, _: *const c_char) = transmute(lib.get(b"libvlc_audio_output_device_set")?);
        let libvlc_audio_toggle_mute: extern "C" fn(_: *mut libvlc_media_player_t) = transmute(lib.get(b"libvlc_audio_toggle_mute")?);
        let libvlc_audio_get_mute: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_audio_get_mute")?);
        let libvlc_audio_set_mute: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) = transmute(lib.get(b"libvlc_audio_set_mute")?);
        let libvlc_audio_get_volume: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_audio_get_volume")?);
        let libvlc_audio_set_volume: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_audio_set_volume")?);
        let libvlc_audio_get_track_count: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_audio_get_track_count")?);
        let libvlc_audio_get_track_description: extern "C" fn(_: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t = transmute(lib.get(b"libvlc_audio_get_track_description")?);
        let libvlc_audio_get_track: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_audio_get_track")?);
        let libvlc_audio_set_track: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_audio_set_track")?);
        let libvlc_audio_get_channel: extern "C" fn(_: *mut libvlc_media_player_t) -> c_int = transmute(lib.get(b"libvlc_audio_get_channel")?);
        let libvlc_audio_set_channel: extern "C" fn(_: *mut libvlc_media_player_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_audio_set_channel")?);
        let libvlc_audio_get_delay: extern "C" fn(_: *mut libvlc_media_player_t) -> i64 = transmute(lib.get(b"libvlc_audio_get_delay")?);
        let libvlc_audio_set_delay: extern "C" fn(_: *mut libvlc_media_player_t, _: i64) -> c_int = transmute(lib.get(b"libvlc_audio_set_delay")?);
        let libvlc_audio_equalizer_get_preset_count: extern "C" fn() -> c_uint = transmute(lib.get(b"libvlc_audio_equalizer_get_preset_count")?);
        let libvlc_audio_equalizer_get_preset_name: extern "C" fn(_: c_uint) -> *const c_char = transmute(lib.get(b"libvlc_audio_equalizer_get_preset_name")?);
        let libvlc_audio_equalizer_get_band_count: extern "C" fn() -> c_uint = transmute(lib.get(b"libvlc_audio_equalizer_get_band_count")?);
        let libvlc_audio_equalizer_get_band_frequency: extern "C" fn(_: c_uint) -> c_float = transmute(lib.get(b"libvlc_audio_equalizer_get_band_frequency")?);
        let libvlc_audio_equalizer_new: extern "C" fn() -> *mut libvlc_equalizer_t = transmute(lib.get(b"libvlc_audio_equalizer_new")?);
        let libvlc_audio_equalizer_new_from_preset: extern "C" fn(_: c_uint) -> *mut libvlc_equalizer_t = transmute(lib.get(b"libvlc_audio_equalizer_new_from_preset")?);
        let libvlc_audio_equalizer_release: extern "C" fn(_: *mut libvlc_equalizer_t) = transmute(lib.get(b"libvlc_audio_equalizer_release")?);
        let libvlc_audio_equalizer_set_preamp: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_float) -> c_int = transmute(lib.get(b"libvlc_audio_equalizer_set_preamp")?);
        let libvlc_audio_equalizer_get_preamp: extern "C" fn(_: *mut libvlc_equalizer_t) -> c_float = transmute(lib.get(b"libvlc_audio_equalizer_get_preamp")?);
        let libvlc_audio_equalizer_set_amp_at_index: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_float, _: c_uint) -> c_int = transmute(lib.get(b"libvlc_audio_equalizer_set_amp_at_index")?);
        let libvlc_audio_equalizer_get_amp_at_index: extern "C" fn(_: *mut libvlc_equalizer_t, _: c_uint) -> c_float = transmute(lib.get(b"libvlc_audio_equalizer_get_amp_at_index")?);
        let libvlc_media_player_set_equalizer: extern "C" fn(_: *mut libvlc_media_player_t, _: *mut libvlc_equalizer_t) -> c_int = transmute(lib.get(b"libvlc_media_player_set_equalizer")?);
        let libvlc_media_list_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_list_t = transmute(lib.get(b"libvlc_media_list_new")?);
        let libvlc_media_list_release: extern "C" fn(_: *mut libvlc_media_list_t) = transmute(lib.get(b"libvlc_media_list_release")?);
        let libvlc_media_list_retain: extern "C" fn(_: *mut libvlc_media_list_t) = transmute(lib.get(b"libvlc_media_list_retain")?);
        let libvlc_media_list_set_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t) = transmute(lib.get(b"libvlc_media_list_set_media")?);
        let libvlc_media_list_media: extern "C" fn(_: *mut libvlc_media_list_t) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_list_media")?);
        let libvlc_media_list_add_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t) -> c_int = transmute(lib.get(b"libvlc_media_list_add_media")?);
        let libvlc_media_list_insert_media: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_media_list_insert_media")?);
        let libvlc_media_list_remove_index: extern "C" fn(_: *mut libvlc_media_list_t, _: c_int) -> c_int = transmute(lib.get(b"libvlc_media_list_remove_index")?);
        let libvlc_media_list_count: extern "C" fn(_: *mut libvlc_media_list_t) -> c_int = transmute(lib.get(b"libvlc_media_list_count")?);
        let libvlc_media_list_item_at_index: extern "C" fn(_: *mut libvlc_media_list_t, _: c_int) -> *mut libvlc_media_t = transmute(lib.get(b"libvlc_media_list_item_at_index")?);
        let libvlc_media_list_index_of_item: extern "C" fn(_: *mut libvlc_media_list_t, _: *mut libvlc_media_t) -> c_int = transmute(lib.get(b"libvlc_media_list_index_of_item")?);
        let libvlc_media_list_is_readonly: extern "C" fn(_: *mut libvlc_media_list_t) -> c_int = transmute(lib.get(b"libvlc_media_list_is_readonly")?);
        let libvlc_media_list_lock: extern "C" fn(_: *mut libvlc_media_list_t) = transmute(lib.get(b"libvlc_media_list_lock")?);
        let libvlc_media_list_unlock: extern "C" fn(_: *mut libvlc_media_list_t) = transmute(lib.get(b"libvlc_media_list_unlock")?);
        let libvlc_media_list_event_manager: extern "C" fn(_: *mut libvlc_media_list_t) -> *mut libvlc_event_manager_t = transmute(lib.get(b"libvlc_media_list_event_manager")?);
        let libvlc_media_library_new: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_media_library_t = transmute(lib.get(b"libvlc_media_library_new")?);
        let libvlc_media_library_release: extern "C" fn(_: *mut libvlc_media_library_t) = transmute(lib.get(b"libvlc_media_library_release")?);
        let libvlc_media_library_retain: extern "C" fn(_: *mut libvlc_media_library_t) = transmute(lib.get(b"libvlc_media_library_retain")?);
        let libvlc_media_library_load: extern "C" fn(_: *mut libvlc_media_library_t) -> c_int = transmute(lib.get(b"libvlc_media_library_load")?);
        let libvlc_media_library_media_list: extern "C" fn(_: *mut libvlc_media_library_t) -> *mut libvlc_media_list_t = transmute(lib.get(b"libvlc_media_library_media_list")?);
        let libvlc_media_discoverer_new_from_name: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *mut libvlc_media_discoverer_t = transmute(lib.get(b"libvlc_media_discoverer_new_from_name")?);
        let libvlc_media_discoverer_release: extern "C" fn(_: *mut libvlc_media_discoverer_t) = transmute(lib.get(b"libvlc_media_discoverer_release")?);
        let libvlc_media_discoverer_localized_name: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut c_char = transmute(lib.get(b"libvlc_media_discoverer_localized_name")?);
        let libvlc_media_discoverer_media_list: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut libvlc_media_list_t = transmute(lib.get(b"libvlc_media_discoverer_media_list")?);
        let libvlc_media_discoverer_event_manager: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> *mut libvlc_event_manager_t = transmute(lib.get(b"libvlc_media_discoverer_event_manager")?);
        let libvlc_media_discoverer_is_running: extern "C" fn(_: *mut libvlc_media_discoverer_t) -> c_int = transmute(lib.get(b"libvlc_media_discoverer_is_running")?);
        let libvlc_vlm_release: extern "C" fn(_: *mut libvlc_instance_t) = transmute(lib.get(b"libvlc_vlm_release")?);
        let libvlc_vlm_add_broadcast: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_add_broadcast")?);
        let libvlc_vlm_add_vod: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_add_vod")?);
        let libvlc_vlm_del_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_del_media")?);
        let libvlc_vlm_set_enabled: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_set_enabled")?);
        let libvlc_vlm_set_output: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_set_output")?);
        let libvlc_vlm_set_input: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_set_input")?);
        let libvlc_vlm_add_input: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_add_input")?);
        let libvlc_vlm_set_loop: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_set_loop")?);
        let libvlc_vlm_set_mux: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_set_mux")?);
        let libvlc_vlm_change_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: *const c_char, _: *const c_char, _: c_int, _: *const *const c_char, _: c_int, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_change_media")?);
        let libvlc_vlm_play_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_play_media")?);
        let libvlc_vlm_stop_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_stop_media")?);
        let libvlc_vlm_pause_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> c_int = transmute(lib.get(b"libvlc_vlm_pause_media")?);
        let libvlc_vlm_seek_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_float) -> c_int = transmute(lib.get(b"libvlc_vlm_seek_media")?);
        let libvlc_vlm_show_media: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char) -> *const c_char = transmute(lib.get(b"libvlc_vlm_show_media")?);
        let libvlc_vlm_get_media_instance_position: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_float = transmute(lib.get(b"libvlc_vlm_get_media_instance_position")?);
        let libvlc_vlm_get_media_instance_time: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_get_media_instance_time")?);
        let libvlc_vlm_get_media_instance_length: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_get_media_instance_length")?);
        let libvlc_vlm_get_media_instance_rate: extern "C" fn(_: *mut libvlc_instance_t, _: *const c_char, _: c_int) -> c_int = transmute(lib.get(b"libvlc_vlm_get_media_instance_rate")?);
        let libvlc_vlm_get_event_manager: extern "C" fn(_: *mut libvlc_instance_t) -> *mut libvlc_event_manager_t = transmute(lib.get(b"libvlc_vlm_get_event_manager")?);
        Some(VlcDll {
            lib,
            libvlc_errmsg,
            libvlc_clearerr,
            libvlc_new,
            libvlc_release,
            libvlc_retain,
            libvlc_add_intf,
            libvlc_set_exit_handler,
            libvlc_wait,
            libvlc_set_user_agent,
            libvlc_set_app_id,
            libvlc_get_version,
            libvlc_get_compiler,
            libvlc_get_changeset,
            libvlc_free,
            libvlc_event_attach,
            libvlc_event_type_name,
            libvlc_log_get_context,
            libvlc_log_get_object,
            libvlc_log_unset,
            libvlc_log_set,
            libvlc_log_set_file,
            libvlc_module_description_list_release,
            libvlc_audio_filter_list_get,
            libvlc_video_filter_list_get,
            libvlc_clock,
            libvlc_media_new_location,
            libvlc_media_new_path,
            libvlc_media_new_fd,
            libvlc_media_add_option,
            libvlc_media_add_option_flag,
            libvlc_media_retain,
            libvlc_media_release,
            libvlc_media_get_mrl,
            libvlc_media_duplicate,
            libvlc_media_get_meta,
            libvlc_media_set_meta,
            libvlc_media_save_meta,
            libvlc_media_get_state,
            libvlc_media_get_stats,
            libvlc_media_subitems,
            libvlc_media_event_manager,
            libvlc_media_get_duration,
            libvlc_media_parse,
            libvlc_media_parse_async,
            libvlc_media_is_parsed,
            libvlc_media_set_user_data,
            libvlc_media_get_user_data,
            libvlc_media_tracks_get,
            libvlc_media_tracks_release,
            libvlc_media_player_new,
            libvlc_media_player_new_from_media,
            libvlc_media_player_release,
            libvlc_media_player_retain,
            libvlc_media_player_set_media,
            libvlc_media_player_get_media,
            libvlc_media_player_event_manager,
            libvlc_media_player_is_playing,
            libvlc_media_player_play,
            libvlc_media_player_set_pause,
            libvlc_media_player_pause,
            libvlc_media_player_stop,
            libvlc_video_set_callbacks,
            libvlc_video_set_format,
            libvlc_video_set_format_callbacks,
            libvlc_media_player_set_nsobject,
            libvlc_media_player_get_nsobject,
            libvlc_media_player_set_xwindow,
            libvlc_media_player_get_xwindow,
            libvlc_media_player_set_hwnd,
            libvlc_media_player_get_hwnd,
            libvlc_audio_set_callbacks,
            libvlc_audio_set_volume_callback,
            libvlc_audio_set_format_callbacks,
            libvlc_audio_set_format,
            libvlc_media_player_get_length,
            libvlc_media_player_get_time,
            libvlc_media_player_set_time,
            libvlc_media_player_get_position,
            libvlc_media_player_set_position,
            libvlc_media_player_set_chapter,
            libvlc_media_player_get_chapter,
            libvlc_media_player_get_chapter_count,
            libvlc_media_player_will_play,
            libvlc_media_player_set_title,
            libvlc_media_player_get_chapter_count_for_title,
            libvlc_media_player_get_title,
            libvlc_media_player_get_title_count,
            libvlc_media_player_previous_chapter,
            libvlc_media_player_next_chapter,
            libvlc_media_player_get_rate,
            libvlc_media_player_set_rate,
            libvlc_media_player_get_state,
            libvlc_media_player_get_fps,
            libvlc_media_player_has_vout,
            libvlc_media_player_is_seekable,
            libvlc_media_player_can_pause,
            libvlc_media_player_program_scrambled,
            libvlc_media_player_next_frame,
            libvlc_media_player_navigate,
            libvlc_media_player_set_video_title_display,
            libvlc_track_description_list_release,
            libvlc_toggle_fullscreen,
            libvlc_set_fullscreen,
            libvlc_get_fullscreen,
            libvlc_video_set_key_input,
            libvlc_video_set_mouse_input,
            libvlc_video_get_size,
            libvlc_video_get_cursor,
            libvlc_video_get_scale,
            libvlc_video_set_scale,
            libvlc_video_get_aspect_ratio,
            libvlc_video_set_aspect_ratio,
            libvlc_video_get_spu,
            libvlc_video_get_spu_count,
            libvlc_video_get_spu_description,
            libvlc_video_set_spu,
            libvlc_video_set_subtitle_file,
            libvlc_video_get_spu_delay,
            libvlc_video_set_spu_delay,
            libvlc_video_get_title_description,
            libvlc_video_get_chapter_description,
            libvlc_video_get_crop_geometry,
            libvlc_video_set_crop_geometry,
            libvlc_video_get_teletext,
            libvlc_video_set_teletext,
            libvlc_toggle_teletext,
            libvlc_video_get_track_count,
            libvlc_video_get_track_description,
            libvlc_video_get_track,
            libvlc_video_set_track,
            libvlc_video_take_snapshot,
            libvlc_video_set_deinterlace,
            libvlc_video_get_marquee_int,
            libvlc_video_get_marquee_string,
            libvlc_video_set_marquee_int,
            libvlc_video_set_marquee_string,
            libvlc_video_get_logo_int,
            libvlc_video_set_logo_int,
            libvlc_video_set_logo_string,
            libvlc_video_get_adjust_int,
            libvlc_video_set_adjust_int,
            libvlc_video_get_adjust_float,
            libvlc_video_set_adjust_float,
            libvlc_audio_output_list_get,
            libvlc_audio_output_list_release,
            libvlc_audio_output_set,
            libvlc_audio_output_device_enum,
            libvlc_audio_output_device_list_get,
            libvlc_audio_output_device_list_release,
            libvlc_audio_output_device_set,
            libvlc_audio_toggle_mute,
            libvlc_audio_get_mute,
            libvlc_audio_set_mute,
            libvlc_audio_get_volume,
            libvlc_audio_set_volume,
            libvlc_audio_get_track_count,
            libvlc_audio_get_track_description,
            libvlc_audio_get_track,
            libvlc_audio_set_track,
            libvlc_audio_get_channel,
            libvlc_audio_set_channel,
            libvlc_audio_get_delay,
            libvlc_audio_set_delay,
            libvlc_audio_equalizer_get_preset_count,
            libvlc_audio_equalizer_get_preset_name,
            libvlc_audio_equalizer_get_band_count,
            libvlc_audio_equalizer_get_band_frequency,
            libvlc_audio_equalizer_new,
            libvlc_audio_equalizer_new_from_preset,
            libvlc_audio_equalizer_release,
            libvlc_audio_equalizer_set_preamp,
            libvlc_audio_equalizer_get_preamp,
            libvlc_audio_equalizer_set_amp_at_index,
            libvlc_audio_equalizer_get_amp_at_index,
            libvlc_media_player_set_equalizer,
            libvlc_media_list_new,
            libvlc_media_list_release,
            libvlc_media_list_retain,
            libvlc_media_list_set_media,
            libvlc_media_list_media,
            libvlc_media_list_add_media,
            libvlc_media_list_insert_media,
            libvlc_media_list_remove_index,
            libvlc_media_list_count,
            libvlc_media_list_item_at_index,
            libvlc_media_list_index_of_item,
            libvlc_media_list_is_readonly,
            libvlc_media_list_lock,
            libvlc_media_list_unlock,
            libvlc_media_list_event_manager,
            libvlc_media_library_new,
            libvlc_media_library_release,
            libvlc_media_library_retain,
            libvlc_media_library_load,
            libvlc_media_library_media_list,
            libvlc_media_discoverer_new_from_name,
            libvlc_media_discoverer_release,
            libvlc_media_discoverer_localized_name,
            libvlc_media_discoverer_media_list,
            libvlc_media_discoverer_event_manager,
            libvlc_media_discoverer_is_running,
            libvlc_vlm_release,
            libvlc_vlm_add_broadcast,
            libvlc_vlm_add_vod,
            libvlc_vlm_del_media,
            libvlc_vlm_set_enabled,
            libvlc_vlm_set_output,
            libvlc_vlm_set_input,
            libvlc_vlm_add_input,
            libvlc_vlm_set_loop,
            libvlc_vlm_set_mux,
            libvlc_vlm_change_media,
            libvlc_vlm_play_media,
            libvlc_vlm_stop_media,
            libvlc_vlm_pause_media,
            libvlc_vlm_seek_media,
            libvlc_vlm_show_media,
            libvlc_vlm_get_media_instance_position,
            libvlc_vlm_get_media_instance_time,
            libvlc_vlm_get_media_instance_length,
            libvlc_vlm_get_media_instance_rate,
            libvlc_vlm_get_event_manager,
        })
    }
}

use std::{mem::MaybeUninit, sync::atomic::{AtomicBool, Ordering}};

static LIBRARY_IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static mut VLC_DLL: MaybeUninit<VlcDll> = MaybeUninit::<VlcDll>::uninit();

#[cfg(target_os="linux")]
static LIB_DEPS: [(&[u8], &str);3] = [
    (include_bytes!("../dll/3.0.11/linux/libvlc.so"), "libvlc.so"),
    (include_bytes!("../dll/3.0.11/linux/libvlccore.so"), "libvlccore.so"),
    (include_bytes!("../dll/3.0.11/linux/axvlc.so"), "axvlc.so"),
];

#[cfg(target_os="windows")]
static LIB_DEPS: [(&[u8], &str);3] = [
    (include_bytes!("../dll/3.0.11/windows/libvlc.dll"), "libvlc.dll"),
    (include_bytes!("../dll/3.0.11/windows/libvlccore.dll"), "libvlccore.dll"),
    (include_bytes!("../dll/3.0.11/windows/axvlc.dll"), "axvlc.dll"),
];

#[cfg(target_os="macos")]
static LIB_DEPS: [(&[u8], &str);3] = [
    (include_bytes!("../dll/3.0.11/macos/libvlc.dynlib"), "libvlc.dynlib"),
    (include_bytes!("../dll/3.0.11/macos/libvlccore.dynlib"), "libvlccore.dynlib"),
    (include_bytes!("../dll/3.0.11/macos/axvlc.dynlib"), "axvlc.dynlib"),
];

fn load_library_inner() -> Result<VlcDll, &'static str> {

    let current_exe_path = std::env::current_exe().map_err(|_| "current exe has no current dir (?!)")?;
    let mut library_path = current_exe_path.parent().ok_or("current exe has no parent (?!)")?.to_path_buf();

    for (lib_bytes, dll_file_name) in LIB_DEPS.iter() {
        let mut library_path = library_path.clone();
        library_path.push(dll_file_name);

        if !library_path.exists() {
           std::fs::write(&library_path, lib_bytes).map_err(|_| "could not unpack DLL")?;
        }
    }

    library_path.push(LIB_DEPS[0].1);

    initialize_library(&library_path).ok_or("could not initialize library")
}

pub(crate) fn get_vlc_dll() -> &'static VlcDll {
    if !LIBRARY_IS_INITIALIZED.load(Ordering::SeqCst) {
       match load_library_inner() {
           Ok(s) => {
               unsafe { VLC_DLL = MaybeUninit::new(s) };
               LIBRARY_IS_INITIALIZED.store(true, Ordering::SeqCst);
           },
           Err(e) => { println!("failed to initialize libvlc dll: missing function {}", e); std::process::exit(-1); }
       }
    }

    unsafe { &*VLC_DLL.as_ptr() }
}
