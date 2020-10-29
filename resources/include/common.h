typedef ptrdiff_t php_int;

// PHP objects are exposed to FFI as a php_object.
// This counts as a reference to the object,
// and free_object() must be explicitly call to free the reference.
// php_object values passed to FFI must be freed exactly the same number of times as passed.
typedef php_int php_object;

// PHP strings are passed over FFI as a char pointer.
// When passed from PHP, it must be free_string()ed explicitly as long as unused.
// When passed to PHP, it must be NUL-terminated and live for at least the time of the function call.
typedef const char * php_string;

// This type represents strings that cannot leak.
//
// When used in parameters, this implies the pointer is only valid before the function returns.
// When used in return value, this implies the string is static and always valid.
typedef const char * string;

// Lists of PHP string are passed over FFI as a pointer to the first string and a size.
// This has identical memory semantics as a string.
// The strings are NUL-terminated in contiguous memory.
// ptr can be anything if size is 0.
typedef struct {
	const char *ptr;
	size_t size;
} string_list;

typedef struct {
	void (*free_object)(php_object);
	void (*free_string)(string);
} ffi_api;

char errstr[4096];
