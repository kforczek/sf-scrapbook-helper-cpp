#include <iostream>
#include <unordered_set>
#include "sf_session.h"


void printResponse(const sf::CommandResponse& response)
{
    std::unordered_set<std::string> keys = response.getKeys();
    std::cout << "Keys count: " << keys.size() << "\n";

    for (const std::string& key : keys)
    {
        std::cout << "Key: " << key << "; Value: " << response.getValue(key);
    }
}

int main() {
    const std::string username = "";
    const std::string password = "";
    const std::string serverUrl = "";

    sf::Session session{ username, password, serverUrl };
    session.update();

    sf::CommandResponse hallOfFame = session.hallOfFamePage(/*page*/ 0);
    printResponse(hallOfFame);

    return 0;
}