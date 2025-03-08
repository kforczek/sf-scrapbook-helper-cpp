#include "sf-api.hpp"
#include <iostream>

void print_vals(ffi::Response* response)
{
    size_t keys_cnt;
    const int8_t** keys_raw = ffi::sf_response_get_keys(response, &keys_cnt);

    auto** keys = reinterpret_cast<const char**>(keys_raw);

    std::cout << "Keys cnt: " << keys_cnt << "\n";
    std::cout << "Response:\n";
    for (size_t i = 0; i < keys_cnt; ++i) {
        std::cout << "Key: " << keys[i] << "; ";
        
        int8_t* val_raw = ffi::sf_response_get_value(response, keys_raw[i]);
        std::cout << "Val: " << reinterpret_cast<char*>(val_raw) << "\n";
        
        ffi::sf_response_free_value(val_raw);
    }

    ffi::sf_response_free_keys(keys_raw, keys_cnt);
}

int main() {
    const char* username = "";
    const char* password = "";
    const char* server_url = "http://s17.sfgame.eu";

    ffi::Session* session = ffi::sf_session_new(reinterpret_cast<const int8_t*>(username),
                                      reinterpret_cast<const int8_t*>(password),
                                      reinterpret_cast<const int8_t*>(server_url));

    if (!session) {
        std::cerr << "Failed to create session\n";
        return 1;
    }

    if (ffi::sf_session_login(session)) {
        std::cout << "Login successful!\n";
    } else {
        std::cout << "Login failed!\n";
        return 1;
    }

    ffi::Response* response = ffi::sf_command_execute(session, 1);
    ffi::sf_response_free(response);

    response = ffi::sf_command_execute(session, 31);
    
    print_vals(response);

    ffi::sf_response_free(response);
    ffi::sf_session_free(session);
    return 0;
}