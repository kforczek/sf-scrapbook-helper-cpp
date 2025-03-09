#include "sf-api.hpp"
#include <iostream>

void print_vals(ffi::Response* response)
{
    size_t keys_cnt;
    const int8_t** keys_raw = ffi::response_get_keys(response, &keys_cnt);

    auto** keys = reinterpret_cast<const char**>(keys_raw);

    std::cout << "Keys cnt: " << keys_cnt << "\n";
    std::cout << "Response:\n";
    for (size_t i = 0; i < keys_cnt; ++i) {
        std::cout << "Key: " << keys[i] << "; ";
        
        int8_t* val_raw = ffi::response_get_value(response, keys_raw[i]);
        std::cout << "Val: " << reinterpret_cast<char*>(val_raw) << "\n";
        
        ffi::destr_response_value(val_raw);
    }

    ffi::destr_response_keys(keys_raw, keys_cnt);
}

int main() {
    const char* username = "E52Yi0yM";
    const char* password = "My0iY25E";
    const char* server_url = "http://s17.sfgame.eu";

    ffi::Session* session = ffi::init_session(reinterpret_cast<const int8_t*>(username),
                                      reinterpret_cast<const int8_t*>(password),
                                      reinterpret_cast<const int8_t*>(server_url));

    if (!session) {
        std::cerr << "Failed to create session\n";
        return 1;
    }

    if (ffi::login(session)) {
        std::cout << "Login successful!\n";
    } else {
        std::cout << "Login failed!\n";
        return 1;
    }

    ffi::Response* response = ffi::exec_Update(session);
    ffi::destr_response(response);

    response = ffi::exec_HallOfFamePage(session, 0);
    
    print_vals(response);

    ffi::destr_response(response);
    ffi::destr_session(session);
    return 0;
}