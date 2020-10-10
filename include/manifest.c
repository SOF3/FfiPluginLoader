string plugin_name();
string plugin_version();
string plugin_ffi_version();


typedef struct {
	ffi_api ffi;
	command_api command;
} api;

void plugin_entry_point(api *api);
