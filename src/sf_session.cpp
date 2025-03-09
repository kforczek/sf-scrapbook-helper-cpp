#include "sf_session.h"
#include "sf-api.hpp"

namespace
{

using ffi_char_t = int8_t;

const ffi_char_t* to_ffi_str(const std::string& str)
{
    return reinterpret_cast<const ffi_char_t*>(str.c_str());
}

std::string to_str(const ffi_char_t* ffiStr)
{
    return reinterpret_cast<const char*>(ffiStr);
}

} // namespace


namespace sf
{

Session::Session(const std::string& username, const std::string& password, const std::string& serverUrl)
{
    m_session = ffi::init_session(to_ffi_str(username), to_ffi_str(password), to_ffi_str(serverUrl));
    if (!m_session)
        throw std::runtime_error("Failed to create session");

    bool isLoginSuccessful = ffi::login(m_session);
    if (!isLoginSuccessful)
        throw std::runtime_error("Invalid login credentials");
}

Session::~Session()
{
    if (m_session)
        ffi::destr_session(m_session);
}

CommandResponse Session::update()
{
    return ffi::exec_Update(m_session);
}

CommandResponse Session::hallOfFamePage(size_t page)
{
    return ffi::exec_HallOfFamePage(m_session, page);
}

CommandResponse::CommandResponse(ffi::Response* response)
    : m_response(response) { }

CommandResponse::~CommandResponse()
{
    if (m_response)
        ffi::destr_response(m_response);
}

bool CommandResponse::isValid() const
{
    return m_response != nullptr;
}

std::unordered_set<std::string> CommandResponse::getKeys() const
{
    size_t keysCnt;
    const ffi_char_t** rawKeys = ffi::response_get_keys(m_response, &keysCnt);

    std::unordered_set<std::string> set;
    for (size_t i = 0; i < keysCnt; ++i)
        set.insert(to_str(rawKeys[i]));

    ffi::destr_response_keys(rawKeys, keysCnt);
    return set;
}

std::string CommandResponse::getValue(const std::string& key) const
{
    ffi_char_t* rawVal = ffi::response_get_value(m_response, to_ffi_str(key));
    
    std::string val = to_str(rawVal);
    ffi::destr_response_value(rawVal);

    return val;
}

} // namespace sf