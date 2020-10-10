typedef enum {
	COMMAND_SENDER_TYPE_CONSOLE = 0,
	COMMAND_SENDER_TYPE_PLAYER = 1,
	COMMAND_SENDER_TYPE_OTHER = 2,
} command_sender_type;

typedef struct {
	php_object value;
	string language;
	string name;
	uint64_t screen_line_height;
} command_sender;

typedef struct {
	bool (*has_permission)(php_object sender, string permission);
	void (*send_message)(php_object sender);
	void (*register)(string name, string description, string usage, string_list aliases, string permission);
} command_api;
