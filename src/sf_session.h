#include <string>
#include <unordered_set>


namespace ffi
{
    class Session;
    class Response;
}

namespace sf
{

class CommandResponse;


class Session
{
public:
    Session(const std::string& username, const std::string& password, const std::string& serverUrl);
    ~Session();

    CommandResponse update();
    CommandResponse hallOfFamePage(size_t page);

private:
    ffi::Session* m_session;
};


class CommandResponse
{
public:
    CommandResponse(ffi::Response* response);
    ~CommandResponse();

    bool isValid() const;

    std::unordered_set<std::string> getKeys() const;
    std::string getValue(const std::string& key) const;

private:
    ffi::Response* m_response;
};

} // namespace sf
