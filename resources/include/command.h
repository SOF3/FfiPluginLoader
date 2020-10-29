typedef enum {
	COMMAND_SENDER_TYPE_CONSOLE = 0,
	COMMAND_SENDER_TYPE_PLAYER = 1,
	COMMAND_SENDER_TYPE_OTHER = 2,
} command_sender_type;

typedef struct {
	php_object value;
	string language;
	string name;
	php_int screen_line_height;
} command_sender;

typedef struct {
	void (*register_command)(string name, string description, string usage, string_list aliases, string permission);
	void (*handle_command)(string name, php_object command_sender, string_list args);
	void (*send_message)(command_sender, string message);
} command_api;
