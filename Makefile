gcc_headers_search_path:
	echo | gcc -E -Wp,-v -
gcc_linker_library_search_path:
	ld --verbose | grep SEARCH_DIR | tr -s ' ;'
