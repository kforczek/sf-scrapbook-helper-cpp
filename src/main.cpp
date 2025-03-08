#include "sf-api.hpp"
#include <iostream>

int main() {
    const char* username = "";
    const char* password = "";
    const char* server_url = "http://s17.sfgame.eu";

    ffi::Session* session = ffi::sf_session_new(reinterpret_cast<const uint8_t*>(username),
                                      reinterpret_cast<const uint8_t*>(password),
                                      reinterpret_cast<const uint8_t*>(server_url));

    if (!session) {
        std::cerr << "Failed to create session\n";
        return 1;
    }

    if (ffi::sf_session_login(session)) {
        std::cout << "Login successful!\n";
    } else {
        std::cout << "Login failed!\n";
    }

    ffi::sf_session_free(session);
    return 0;
}