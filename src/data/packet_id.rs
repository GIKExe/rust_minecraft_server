/*

Generated with parse_ids.py

*/

pub mod clientbound {
	pub mod status {
		pub const RESPONSE: u8 = 0x00;
		pub const PONG_RESPONSE: u8 = 0x01;
	}

	pub mod login {
		pub const DISCONNECT: u8 = 0x00;
		pub const ENCRYPTION_REQUEST: u8 = 0x01;
		pub const SUCCESS: u8 = 0x02;
		pub const SET_COMPRESSION: u8 = 0x03;
		pub const PLUGIN_REQUEST: u8 = 0x04;
		pub const COOKIE_REQUEST: u8 = 0x05;
	}

	pub mod configuration {
		pub const COOKIE_REQUEST: u8 = 0x00;
		pub const PLUGIN_MESSAGE: u8 = 0x01;
		pub const DISCONNECT: u8 = 0x02;
		pub const FINISH: u8 = 0x03;
		pub const KEEP_ALIVE: u8 = 0x04;
		pub const PING: u8 = 0x05;
		pub const RESET_CHAT: u8 = 0x06;
		pub const REGISTRY_DATA: u8 = 0x07;
		pub const REMOVE_RESOURCE_PACK: u8 = 0x08;
		pub const ADD_RESOURCE_PACK: u8 = 0x09;
		pub const STORE_COOKIE: u8 = 0x0A;
		pub const TRANSFER: u8 = 0x0B;
		pub const FEATURE_FLAGS: u8 = 0x0C;
		pub const UPDATE_TAGS: u8 = 0x0D;
		pub const KNOWN_PACKS: u8 = 0x0E;
		pub const CUSTOM_REPORT_DETAILS: u8 = 0x0F;
		pub const SERVER_LINKS: u8 = 0x10;
	}

	pub mod play {
		pub const BUNDLE_DELIMITER: u8 = 0x00;
		pub const SPAWN_ENTITY: u8 = 0x01;
		pub const ENTITY_ANIMATION: u8 = 0x02;
		pub const AWARD_STATISTICS: u8 = 0x03;
		pub const ACKNOWLEDGE_BLOCK_CHANGE: u8 = 0x04;
		pub const SET_BLOCK_DESTROY_STAGE: u8 = 0x05;
		pub const BLOCK_ENTITY_DATA: u8 = 0x06;
		pub const BLOCK_ACTION: u8 = 0x07;
		pub const BLOCK_UPDATE: u8 = 0x08;
		pub const BOSS_BAR: u8 = 0x09;
		pub const CHANGE_DIFFICULTY: u8 = 0x0A;
		pub const CHUNK_BATCH_FINISHED: u8 = 0x0B;
		pub const CHUNK_BATCH_START: u8 = 0x0C;
		pub const CHUNK_BIOMES: u8 = 0x0D;
		pub const CLEAR_TITLES: u8 = 0x0E;
		pub const COMMAND_SUGGESTIONS_RESPONSE: u8 = 0x0F;
		pub const COMMANDS: u8 = 0x10;
		pub const CLOSE_CONTAINER: u8 = 0x11;
		pub const SET_CONTAINER_CONTENT: u8 = 0x12;
		pub const SET_CONTAINER_PROPERTY: u8 = 0x13;
		pub const SET_CONTAINER_SLOT: u8 = 0x14;
		pub const COOKIE_REQUEST: u8 = 0x15;
		pub const SET_COOLDOWN: u8 = 0x16;
		pub const CHAT_SUGGESTIONS: u8 = 0x17;
		pub const PLUGIN_MESSAGE: u8 = 0x18;
		pub const DAMAGE_EVENT: u8 = 0x19;
		pub const DEBUG_SAMPLE: u8 = 0x1A;
		pub const DELETE_MESSAGE: u8 = 0x1B;
		pub const DISCONNECT: u8 = 0x1C;
		pub const DISGUISED_CHAT_MESSAGE: u8 = 0x1D;
		pub const ENTITY_EVENT: u8 = 0x1E;
		pub const TELEPORT_ENTITY: u8 = 0x1F;
		pub const EXPLOSION: u8 = 0x20;
		pub const UNLOAD_CHUNK: u8 = 0x21;
		pub const GAME_EVENT: u8 = 0x22;
		pub const OPEN_HORSE_SCREEN: u8 = 0x23;
		pub const HURT_ANIMATION: u8 = 0x24;
		pub const INITIALIZE_WORLD_BORDER: u8 = 0x25;
		pub const KEEP_ALIVE: u8 = 0x26;
		pub const CHUNK_DATA_AND_UPDATE_LIGHT: u8 = 0x27;
		pub const WORLD_EVENT: u8 = 0x28;
		pub const PARTICLE: u8 = 0x29;
		pub const UPDATE_LIGHT: u8 = 0x2A;
		pub const LOGIN: u8 = 0x2B;
		pub const MAP_DATA: u8 = 0x2C;
		pub const MERCHANT_OFFERS: u8 = 0x2D;
		pub const UPDATE_ENTITY_POSITION: u8 = 0x2E;
		pub const UPDATE_ENTITY_POSITION_AND_ROTATION: u8 = 0x2F;
		pub const MOVE_MINECART_ALONG_TRACK: u8 = 0x30;
		pub const UPDATE_ENTITY_ROTATION: u8 = 0x31;
		pub const MOVE_VEHICLE: u8 = 0x32;
		pub const OPEN_BOOK: u8 = 0x33;
		pub const OPEN_SCREEN: u8 = 0x34;
		pub const OPEN_SIGN_EDITOR: u8 = 0x35;
		pub const PING: u8 = 0x36;
		pub const PING_RESPONSE: u8 = 0x37;
		pub const PLACE_GHOST_RECIPE: u8 = 0x38;
		pub const PLAYER_ABILITIES: u8 = 0x39;
		pub const PLAYER_CHAT_MESSAGE: u8 = 0x3A;
		pub const END_COMBAT: u8 = 0x3B;
		pub const ENTER_COMBAT: u8 = 0x3C;
		pub const COMBAT_DEATH: u8 = 0x3D;
		pub const PLAYER_INFO_REMOVE: u8 = 0x3E;
		pub const PLAYER_INFO_UPDATE: u8 = 0x3F;
		pub const LOOK_AT: u8 = 0x40;
		pub const SYNCHRONIZE_PLAYER_POSITION: u8 = 0x41;
		pub const PLAYER_ROTATION: u8 = 0x42;
		pub const RECIPE_BOOK_ADD: u8 = 0x43;
		pub const RECIPE_BOOK_REMOVE: u8 = 0x44;
		pub const RECIPE_BOOK_SETTINGS: u8 = 0x45;
		pub const REMOVE_ENTITIES: u8 = 0x46;
		pub const REMOVE_ENTITY_EFFECT: u8 = 0x47;
		pub const RESET_SCORE: u8 = 0x48;
		pub const REMOVE_RESOURCE_PACK: u8 = 0x49;
		pub const ADD_RESOURCE_PACK: u8 = 0x4A;
		pub const RESPAWN: u8 = 0x4B;
		pub const SET_HEAD_ROTATION: u8 = 0x4C;
		pub const UPDATE_SECTION_BLOCKS: u8 = 0x4D;
		pub const SELECT_ADVANCEMENTS_TAB: u8 = 0x4E;
		pub const SERVER_DATA: u8 = 0x4F;
		pub const SET_ACTION_BAR_TEXT: u8 = 0x50;
		pub const SET_BORDER_CENTER: u8 = 0x51;
		pub const SET_BORDER_LERP_SIZE: u8 = 0x52;
		pub const SET_BORDER_SIZE: u8 = 0x53;
		pub const SET_BORDER_WARNING_DELAY: u8 = 0x54;
		pub const SET_BORDER_WARNING_DISTANCE: u8 = 0x55;
		pub const SET_CAMERA: u8 = 0x56;
		pub const SET_CENTER_CHUNK: u8 = 0x57;
		pub const SET_RENDER_DISTANCE: u8 = 0x58;
		pub const SET_CURSOR_ITEM: u8 = 0x59;
		pub const SET_DEFAULT_SPAWN_POSITION: u8 = 0x5A;
		pub const DISPLAY_OBJECTIVE: u8 = 0x5B;
		pub const SET_ENTITY_METADATA: u8 = 0x5C;
		pub const LINK_ENTITIES: u8 = 0x5D;
		pub const SET_ENTITY_VELOCITY: u8 = 0x5E;
		pub const SET_EQUIPMENT: u8 = 0x5F;
		pub const SET_EXPERIENCE: u8 = 0x60;
		pub const SET_HEALTH: u8 = 0x61;
		pub const SET_HELD_ITEM: u8 = 0x62;
		pub const UPDATE_OBJECTIVES: u8 = 0x63;
		pub const SET_PASSENGERS: u8 = 0x64;
		pub const SET_PLAYER_INVENTORY_SLOT: u8 = 0x65;
		pub const UPDATE_TEAMS: u8 = 0x66;
		pub const UPDATE_SCORE: u8 = 0x67;
		pub const SET_SIMULATION_DISTANCE: u8 = 0x68;
		pub const SET_SUBTITLE_TEXT: u8 = 0x69;
		pub const UPDATE_TIME: u8 = 0x6A;
		pub const SET_TITLE_TEXT: u8 = 0x6B;
		pub const SET_TITLE_ANIMATION_TIMES: u8 = 0x6C;
		pub const ENTITY_SOUND_EFFECT: u8 = 0x6D;
		pub const SOUND_EFFECT: u8 = 0x6E;
		pub const START_CONFIGURATION: u8 = 0x6F;
		pub const STOP_SOUND: u8 = 0x70;
		pub const STORE_COOKIE: u8 = 0x71;
		pub const SYSTEM_CHAT_MESSAGE: u8 = 0x72;
		pub const SET_TAB_LIST_HEADER_AND_FOOTER: u8 = 0x73;
		pub const TAG_QUERY_RESPONSE: u8 = 0x74;
		pub const PICKUP_ITEM: u8 = 0x75;
		pub const SYNCHRONIZE_VEHICLE_POSITION: u8 = 0x76;
		pub const TEST_INSTANCE_BLOCK_STATUS: u8 = 0x77;
		pub const SET_TICKING_STATE: u8 = 0x78;
		pub const STEP_TICK: u8 = 0x79;
		pub const TRANSFER: u8 = 0x7A;
		pub const UPDATE_ADVANCEMENTS: u8 = 0x7B;
		pub const UPDATE_ATTRIBUTES: u8 = 0x7C;
		pub const ENTITY_EFFECT: u8 = 0x7D;
		pub const UPDATE_RECIPES: u8 = 0x7E;
		pub const UPDATE_TAGS: u8 = 0x7F;
		pub const PROJECTILE_POWER: u8 = 0x80;
		pub const CUSTOM_REPORT_DETAILS: u8 = 0x81;
		pub const SERVER_LINKS: u8 = 0x82;
	}
}

pub mod serverbound {
	pub mod handshake {
		pub const HANDSHAKE: u8 = 0x00;
	}

	pub mod status {
		pub const REQUEST: u8 = 0x00;
		pub const PING_REQUEST: u8 = 0x01;
	}

	pub mod login {
		pub const START: u8 = 0x00;
		pub const ENCRYPTION_RESPONSE: u8 = 0x01;
		pub const PLUGIN_RESPONSE: u8 = 0x02;
		pub const ACKNOWLEDGED: u8 = 0x03;
		pub const COOKIE_RESPONSE: u8 = 0x04;
	}

	pub mod configuration {
		pub const CLIENT_INFORMATION: u8 = 0x00;
		pub const COOKIE_RESPONSE: u8 = 0x01;
		pub const PLUGIN_MESSAGE: u8 = 0x02;
		pub const ACKNOWLEDGE_FINISH: u8 = 0x03;
		pub const KEEP_ALIVE: u8 = 0x04;
		pub const PONG: u8 = 0x05;
		pub const RESOURCE_PACK_RESPONSE: u8 = 0x06;
		pub const KNOWN_PACKS: u8 = 0x07;
	}

	pub mod play {
		pub const CONFIRM_TELEPORTATION: u8 = 0x00;
		pub const QUERY_BLOCK_ENTITY_TAG: u8 = 0x01;
		pub const BUNDLE_ITEM_SELECTED: u8 = 0x02;
		pub const CHANGE_DIFFICULTY: u8 = 0x03;
		pub const ACKNOWLEDGE_MESSAGE: u8 = 0x04;
		pub const CHAT_COMMAND: u8 = 0x05;
		pub const SIGNED_CHAT_COMMAND: u8 = 0x06;
		pub const CHAT_MESSAGE: u8 = 0x07;
		pub const PLAYER_SESSION: u8 = 0x08;
		pub const CHUNK_BATCH_RECEIVED: u8 = 0x09;
		pub const CLIENT_STATUS: u8 = 0x0A;
		pub const CLIENT_TICK_END: u8 = 0x0B;
		pub const CLIENT_INFORMATION: u8 = 0x0C;
		pub const COMMAND_SUGGESTIONS_REQUEST: u8 = 0x0D;
		pub const ACKNOWLEDGE_CONFIGURATION: u8 = 0x0E;
		pub const CLICK_CONTAINER_BUTTON: u8 = 0x0F;
		pub const CLICK_CONTAINER: u8 = 0x10;
		pub const CLOSE_CONTAINER: u8 = 0x11;
		pub const CHANGE_CONTAINER_SLOT_STATE: u8 = 0x12;
		pub const COOKIE_RESPONSE: u8 = 0x13;
		pub const PLUGIN_MESSAGE: u8 = 0x14;
		pub const DEBUG_SAMPLE_SUBSCRIPTION: u8 = 0x15;
		pub const EDIT_BOOK: u8 = 0x16;
		pub const QUERY_ENTITY_TAG: u8 = 0x17;
		pub const INTERACT: u8 = 0x18;
		pub const JIGSAW_GENERATE: u8 = 0x19;
		pub const KEEP_ALIVE: u8 = 0x1A;
		pub const LOCK_DIFFICULTY: u8 = 0x1B;
		pub const SET_PLAYER_POSITION: u8 = 0x1C;
		pub const SET_PLAYER_POSITION_AND_ROTATION: u8 = 0x1D;
		pub const SET_PLAYER_ROTATION: u8 = 0x1E;
		pub const SET_PLAYER_MOVEMENT_FLAGS: u8 = 0x1F;
		pub const MOVE_VEHICLE: u8 = 0x20;
		pub const PADDLE_BOAT: u8 = 0x21;
		pub const PICK_ITEM_FROM_BLOCK: u8 = 0x22;
		pub const PICK_ITEM_FROM_ENTITY: u8 = 0x23;
		pub const PING_REQUEST: u8 = 0x24;
		pub const PLACE_RECIPE: u8 = 0x25;
		pub const PLAYER_ABILITIES: u8 = 0x26;
		pub const PLAYER_ACTION: u8 = 0x27;
		pub const PLAYER_COMMAND: u8 = 0x28;
		pub const PLAYER_INPUT: u8 = 0x29;
		pub const PLAYER_LOADED: u8 = 0x2A;
		pub const PONG: u8 = 0x2B;
		pub const CHANGE_RECIPE_BOOK_SETTINGS: u8 = 0x2C;
		pub const SET_SEEN_RECIPE: u8 = 0x2D;
		pub const RENAME_ITEM: u8 = 0x2E;
		pub const RESOURCE_PACK_RESPONSE: u8 = 0x2F;
		pub const SEEN_ADVANCEMENTS: u8 = 0x30;
		pub const SELECT_TRADE: u8 = 0x31;
		pub const SET_BEACON_EFFECT: u8 = 0x32;
		pub const SET_HELD_ITEM: u8 = 0x33;
		pub const PROGRAM_COMMAND_BLOCK: u8 = 0x34;
		pub const PROGRAM_COMMAND_BLOCK_MINECART: u8 = 0x35;
		pub const SET_CREATIVE_MODE_SLOT: u8 = 0x36;
		pub const PROGRAM_JIGSAW_BLOCK: u8 = 0x37;
		pub const PROGRAM_STRUCTURE_BLOCK: u8 = 0x38;
		pub const SET_TEST_BLOCK: u8 = 0x39;
		pub const UPDATE_SIGN: u8 = 0x3A;
		pub const SWING_ARM: u8 = 0x3B;
		pub const TELEPORT_TO_ENTITY: u8 = 0x3C;
		pub const TEST_INSTANCE_BLOCK_ACTION: u8 = 0x3D;
		pub const USE_ITEM_ON: u8 = 0x3E;
		pub const USE_ITEM: u8 = 0x3F;
	}
}
